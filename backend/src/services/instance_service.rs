use sqlx::PgPool;
use tokio::task;
use uuid::Uuid;

use crate::{
    models::{
        dto::lab::{CreateLabResponseDto,LabStatusResponseDto,},
        status::{
            EnvironmentStatus,
            InstanceStatus,
        },
    },
    repositories::{
        ctf_repo,
        environment_repo,
        instance_repo,
        scenario_repo,
        task_progress_repo,
        task_repo,

    },
    utils::{
        network,
        virtualbox_manager,
    },
};

pub async fn create_user_lab(
    pool: &PgPool,
    user_id: Uuid,
    scenario_id: Uuid,
) -> Result<CreateLabResponseDto, String> {
    
    let scenario = scenario_repo::find_active_scenario_by_id(
        pool,
        scenario_id,
    )
    .await
    .map_err(|error| {
        format!("Failed to retrieve scenario: {error}")
    })?
    .ok_or_else(|| {
        "Scenario not found or inactive".to_string()
    })?;

    let active_environment =
        environment_repo::find_active_environment(
            pool,
            user_id,
            scenario_id,
        )
        .await
        .map_err(|error| {
            format!(
                "Failed to check active environments: {error}"
            )
        })?;

    if active_environment.is_some() {
        return Err(
            "You already have an active environment for this scenario"
                .to_string(),
        );
    }

    let environment =
        environment_repo::create_environment(
            pool,
            user_id,
            scenario_id,
        )
        .await
        .map_err(|error| {
            if let sqlx::Error::Database(database_error) = &error {
                if database_error.is_unique_violation() {
                    return "You already have an active environment for this scenario"
                        .to_string();
                }
            }

            format!("Failed to create environment: {error}")
        })?;

    let vm_name = format!(
        "lab-{}-{}",
        user_id,
        Uuid::new_v4().as_simple()
    );

    
    let host_ssh_port = network::get_available_port();
    let ssh_port = i32::from(host_ssh_port);

 
    let instance = match instance_repo::create_instance(
        pool,
        environment.id,
        &vm_name,
        ssh_port,
        true,
    )
    .await
    {
        Ok(instance) => instance,

        Err(error) => {
            let _ =
                environment_repo::update_environment_status(
                    pool,
                    environment.id,
                    EnvironmentStatus::Failed,
                )
                .await;

            return Err(format!(
                "Failed to create instance record: {error}"
            ));
        }
    };

    let template_name = scenario.vm_template_name;
    let vm_name_for_virtualbox = vm_name.clone();

  
    let virtual_machine_result =
        task::spawn_blocking(move || -> Result<(), String> {
            virtualbox_manager::clone_vm(
                &template_name,
                &vm_name_for_virtualbox,
            )?;

            virtualbox_manager::start_vm(
                &vm_name_for_virtualbox,
                host_ssh_port,
            )?;

            virtualbox_manager::wait_for_ssh(
                host_ssh_port,
                std::time::Duration::from_secs(120),
            )?;

            Ok(())
        })
        .await
        .map_err(|error| {
            format!(
                "VirtualBox blocking task failed: {error}"
            )
        })?;

    if let Err(error) = virtual_machine_result {
        let _ = instance_repo::update_instance_status(
            pool,
            instance.id,
            InstanceStatus::Failed,
        )
        .await;

        let _ =
            environment_repo::update_environment_status(
                pool,
                environment.id,
                EnvironmentStatus::Failed,
            )
            .await;

        let vm_name_for_cleanup = vm_name.clone();

        let _ = task::spawn_blocking(move || {
            virtualbox_manager::delete_vm(
                &vm_name_for_cleanup,
            )
        })
        .await;

        return Err(format!(
            "Failed to create or start virtual machine: {error}"
        ));
    }

    let running_instance =
        match instance_repo::update_instance_status(
            pool,
            instance.id,
            InstanceStatus::Running,
        )
        .await
        {
            Ok(instance) => instance,

            Err(error) => {
                cleanup_failed_environment(
                    pool,
                    environment.id,
                    instance.id,
                    vm_name.clone(),
                )
                .await;

                return Err(format!(
                    "VM started, but the instance status could not be updated: {error}"
                ));
            }
        };

    if let Err(error) =
        environment_repo::mark_environment_running(
            pool,
            environment.id,
        )
        .await
    {
        cleanup_failed_environment(
            pool,
            environment.id,
            running_instance.id,
            vm_name.clone(),
        )
        .await;

        return Err(format!(
            "VM started, but the environment status could not be updated: {error}"
        ));
    }

    Ok(CreateLabResponseDto {
        environment_id: environment.id,
        instance_id: running_instance.id,
        scenario_id,
        vm_name,
        ssh_port: running_instance.ssh_port,
        status: EnvironmentStatus::Running
            .as_str()
            .to_string(),
    })
}

pub async fn delete_user_lab(
    pool: &PgPool,
    user_id: Uuid,
    environment_id: Uuid,
) -> Result<(), String> {
    let environment =
        environment_repo::find_user_environment_by_id(
            pool,
            environment_id,
            user_id,
        )
        .await
        .map_err(|error| {
            format!("Failed to retrieve environment: {error}")
        })?
        .ok_or_else(|| {
            "Environment not found".to_string()
        })?;

    /*
        Idempotent behavior:
        deleting an already destroyed Environment is successful.
    */
    if environment.status
        == EnvironmentStatus::Destroyed.as_str()
    {
        return Ok(());
    }

    environment_repo::update_environment_status(
        pool,
        environment_id,
        EnvironmentStatus::Stopping,
    )
    .await
    .map_err(|error| {
        format!(
            "Failed to mark environment as stopping: {error}"
        )
    })?;

    let instance =
        instance_repo::find_by_environment_id(
            pool,
            environment_id,
        )
        .await
        .map_err(|error| {
            format!("Failed to retrieve instance: {error}")
        })?;

    /*
        An old or partially-created Environment may have no
        Instance. In that case there is no VM record to delete,
        so we can safely close the Environment.
    */
    let Some(instance) = instance else {
        environment_repo::mark_environment_destroyed(
            pool,
            environment_id,
        )
        .await
        .map_err(|error| {
            format!(
                "Failed to destroy orphan environment: {error}"
            )
        })?;

        return Ok(());
    };

    instance_repo::update_instance_status(
        pool,
        instance.id,
        InstanceStatus::Stopping,
    )
    .await
    .map_err(|error| {
        format!(
            "Failed to mark instance as stopping: {error}"
        )
    })?;

    let vm_name = instance.vm_name.clone();

    let deletion_result =
        task::spawn_blocking(move || {
            virtualbox_manager::delete_vm(&vm_name)
        })
        .await
        .map_err(|error| {
            format!(
                "VirtualBox deletion task failed: {error}"
            )
        })?;

    if let Err(error) = deletion_result {
        let _ = instance_repo::update_instance_status(
            pool,
            instance.id,
            InstanceStatus::Failed,
        )
        .await;

        let _ =
            environment_repo::update_environment_status(
                pool,
                environment_id,
                EnvironmentStatus::Failed,
            )
            .await;

        return Err(format!(
            "Failed to delete virtual machine: {error}"
        ));
    }

    instance_repo::update_instance_status(
        pool,
        instance.id,
        InstanceStatus::Destroyed,
    )
    .await
    .map_err(|error| {
        format!(
            "VM was deleted, but instance status could not be updated: {error}"
        )
    })?;

    environment_repo::mark_environment_destroyed(
        pool,
        environment_id,
    )
    .await
    .map_err(|error| {
        format!(
            "VM was deleted, but environment status could not be updated: {error}"
        )
    })?;

    Ok(())
}

pub async fn verify_and_submit_flag(
    pool: &PgPool,
    user_id: Uuid,
    environment_id: Uuid,
    task_id: Uuid,
    submitted_flag: &str,
) -> Result<String, String> {
    let environment =
        environment_repo::find_user_environment_by_id(
            pool,
            environment_id,
            user_id,
        )
        .await
        .map_err(|error| {
            format!(
                "Failed to retrieve environment: {error}"
            )
        })?
        .ok_or_else(|| {
            "Environment not found".to_string()
        })?;

    if environment.status
        != EnvironmentStatus::Running.as_str()
    {
        return Err(
            "The environment is not currently running"
                .to_string(),
        );
    }

    let belongs_to_scenario =
        task_repo::task_belongs_to_scenario(
            pool,
            task_id,
            environment.scenario_id,
        )
        .await
        .map_err(|error| {
            format!(
                "Failed to validate task scenario: {error}"
            )
        })?;

    if !belongs_to_scenario {
        return Err(
            "The task is not connected to this lab scenario"
                .to_string(),
        );
    }

    let normalized_flag = submitted_flag.trim();

    if normalized_flag.is_empty() {
        return Err(
            "Flag value cannot be empty".to_string(),
        );
    }

    let flag_details = ctf_repo::get_flag(
        pool,
        environment.scenario_id,
        normalized_flag,
    )
    .await
    .map_err(|error| {
        format!("Failed to verify flag: {error}")
    })?;

    let (flag_id, points) = match flag_details {
        Some(details) => details,

        None => {
            return Ok(
                "❌ Incorrect flag. Keep trying!"
                    .to_string(),
            );
        }
    };

    match ctf_repo::submit_and_score(
        pool,
        user_id,
        flag_id,
        points,
    )
    .await
    {
        Ok(_) => {
            task_progress_repo::mark_task_completed(
                pool,
                user_id,
                task_id,
            )
            .await
            .map_err(|error| {
                format!(
                    "Flag was accepted, but task progress could not be updated: {error}"
                )
            })?;

            Ok(format!(
                "✅ Correct! You earned {} points.",
                points
            ))
        }

        Err(sqlx::Error::Database(database_error))
            if database_error.is_unique_violation() =>
        {
            task_progress_repo::mark_task_completed(
                pool,
                user_id,
                task_id,
            )
            .await
            .map_err(|error| {
                format!(
                    "Flag was already solved, but task progress could not be updated: {error}"
                )
            })?;

            Ok(
                "⚠️ You already submitted this flag!"
                    .to_string(),
            )
        }

        Err(error) => Err(format!(
            "Failed to submit flag: {error}"
        )),
    }
}

async fn cleanup_failed_environment(
    pool: &PgPool,
    environment_id: Uuid,
    instance_id: Uuid,
    vm_name: String,
) {
    /*
        Best-effort cleanup.

        We deliberately ignore cleanup errors here because this
        helper is already running after a more important failure.
        The original error should remain the one returned to the
        caller.
    */
    let _ = task::spawn_blocking(move || {
        virtualbox_manager::delete_vm(&vm_name)
    })
    .await;

    let _ = instance_repo::update_instance_status(
        pool,
        instance_id,
        InstanceStatus::Failed,
    )
    .await;

    let _ = environment_repo::update_environment_status(
        pool,
        environment_id,
        EnvironmentStatus::Failed,
    )
    .await;
}

pub async fn get_lab_status(
    pool: &PgPool,
    user_id: Uuid,
    environment_id: Uuid,
) -> Result<LabStatusResponseDto, String> {
    let environment =
        environment_repo::find_user_environment_by_id(
            pool,
            environment_id,
            user_id,
        )
        .await
        .map_err(|error| {
            format!(
                "Failed to retrieve environment: {}",
                error
            )
        })?
        .ok_or_else(|| {
            "Environment not found".to_string()
        })?;

    let instance =
        instance_repo::find_by_environment_id(
            pool,
            environment.id,
        )
        .await
        .map_err(|error| {
            format!(
                "Failed to retrieve instance: {}",
                error
            )
        })?;

    Ok(LabStatusResponseDto {
        environment_id: environment.id,
        scenario_id: environment.scenario_id,
        environment_status: environment.status,

        instance_id: instance
            .as_ref()
            .map(|instance| instance.id),

        vm_name: instance
            .as_ref()
            .map(|instance| instance.vm_name.clone()),

        ssh_port: instance
            .as_ref()
            .and_then(|instance| instance.ssh_port),

        instance_status: instance
            .as_ref()
            .map(|instance| instance.status.clone()),

        is_entry_point: instance
            .as_ref()
            .map(|instance| instance.is_entry_point),

        created_at: environment.created_at,
        started_at: environment.started_at,
        stopped_at: environment.stopped_at,
    })
}

pub async fn get_active_lab(
    pool: &PgPool,
    user_id: Uuid,
    scenario_id: Uuid,
) -> Result<Option<LabStatusResponseDto>, String> {
    let environment =
        environment_repo::find_active_environment(
            pool,
            user_id,
            scenario_id,
        )
        .await
        .map_err(|error| {
            format!(
                "Failed to retrieve active environment: {}",
                error
            )
        })?;

    let Some(environment) = environment else {
        return Ok(None);
    };

    let instance =
        instance_repo::find_by_environment_id(
            pool,
            environment.id,
        )
        .await
        .map_err(|error| {
            format!(
                "Failed to retrieve active instance: {}",
                error
            )
        })?;

    Ok(Some(LabStatusResponseDto {
        environment_id: environment.id,
        scenario_id: environment.scenario_id,
        environment_status: environment.status,

        instance_id: instance
            .as_ref()
            .map(|instance| instance.id),

        vm_name: instance
            .as_ref()
            .map(|instance| instance.vm_name.clone()),

        ssh_port: instance
            .as_ref()
            .and_then(|instance| instance.ssh_port),

        instance_status: instance
            .as_ref()
            .map(|instance| instance.status.clone()),

        is_entry_point: instance
            .as_ref()
            .map(|instance| instance.is_entry_point),

        created_at: environment.created_at,
        started_at: environment.started_at,
        stopped_at: environment.stopped_at,
    }))
}
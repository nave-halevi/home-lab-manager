use sqlx::PgPool;
use tokio::task;
use uuid::Uuid;
use crate::repositories::ctf_repo;


use crate::utils::{virtualbox_manager, network};

pub async fn create_user_lab(
    pool:&PgPool,
    user_id: Uuid,
    scenario_id: Uuid
) -> Result<(Uuid, u16 ), String> {

    let vm_name = format!("lab-{}-{}", user_id,Uuid::new_v4().as_simple());

    let env_id = sqlx::query!(
        r#"
        INSERT INTO environments (user_id, scenario_id, network_name, status)
        VALUES ($1, $2, $3, 'Building')
        RETURNING id
        "#,
        user_id, scenario_id, vm_name
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?
    .id;


    let vm_name_clone = vm_name.clone();
    let host_ssh_port: u16 = network::get_available_port();
    

    let is_created = task::spawn_blocking(move || {
        const BASE_VM: &str = "kali-linux";
        let base_vm = BASE_VM;
        if virtualbox_manager::clone_vm(base_vm, &vm_name_clone) {
            virtualbox_manager::start_vm(&vm_name_clone, host_ssh_port)
        } else{
            false
        }
    })
    .await
    .map_err(|_|"Error accessing VirtualBox".to_string())?;


    if !is_created{
        sqlx::query!("UPDATE environments SET status = 'Failed' WHERE id = $1", env_id)
            .execute(pool).await.ok();
        return Err("Failed to establish the laboratory".to_string());
    }

    sqlx::query!("UPDATE environments SET status = 'Running' WHERE id = $1", env_id)
        .execute(pool).await.ok();

    Ok((env_id ,host_ssh_port))
}

pub async fn delete_user_lab(pool: &PgPool, env_id: Uuid) -> Result<(), String> {

    let record = sqlx::query!("SELECT network_name FROM environments WHERE id = $1", env_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("שגיאת מסד נתונים: {}", e))?;
    
    let vm_name = match record {
        Some(row) => row.network_name,
        None => return Err("המעבדה לא נמצאה במסד הנתונים".to_string()), 
    };

    let vm_name_clone = vm_name.clone();
    task::spawn_blocking(move || {
        virtualbox_manager::delete_vm(&vm_name_clone);
    })
    .await
    .map_err(|_| "התרסקה משימת המחיקה מול VirtualBox".to_string())?;

    sqlx::query!("UPDATE environments SET status = 'Destroyed' WHERE id = $1", env_id)
        .execute(pool)
        .await
        .ok();

    Ok(())
}

pub async fn verify_and_submit_flag(pool: &PgPool, env_id: Uuid, submitted_flag: &str) -> Result<String, String> {

    let env_details = ctf_repo::get_env_details(pool, env_id).await
    .map_err(|e| e.to_string())?;

    let (user_id, scenario_id) = match env_details {
        Some(details) => details,
        None => return Err("Environment not found or already terminated".to_string()),
    };

    let flag_details = ctf_repo::get_flag(pool, scenario_id, submitted_flag).await
    .map_err(|e| e.to_string())?;

    let (flag_id, points) = match flag_details {
        Some(details) => details,
        None => return Ok("❌ Incorrect flag. Keep trying!".to_string()),
    };

    match ctf_repo::submit_and_score(pool, user_id, flag_id, points).await {
        Ok(_) => Ok(format!("✅ Correct! You earned {} points.", points)),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Ok("⚠️ You already submitted this flag!".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}
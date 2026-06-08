use sqlx::PgPool;
use tokio::task;
use uuid::Uuid;

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
    .map_err(|_| "שגיאה בגישה ל-VirtualBox".to_string())?;


    if !is_created{
        sqlx::query!("UPDATE environments SET status = 'Failed' WHERE id = $1", env_id)
            .execute(pool).await.ok();
        return Err("נכשל בהקמת המעבדה".to_string());
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
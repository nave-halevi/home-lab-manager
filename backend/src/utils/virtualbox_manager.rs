use std::process::Command;

pub fn list_existing_vms(){

    println!("מנסה לתקשר עם VirtualBox...");

    let output = Command::new("vboxmanage")
        .arg("list")
        .arg("vms")
        .output()
        .expect("קרתה שגיאה אנושה בניסיון להריץ פקודת מערכת");

    if output.status.success(){
     
        let stdout = String::from_utf8_lossy(&output.stdout);
     
        println!("=== מכונות וירטואליות קיימות ב-VirtualBox ===");
     
        if stdout.trim().is_empty() {
            println!("לא נמצאו מכונות וירטואליות.");
        } else {
            println!("{}", stdout);
        }

    println!("===============================================");
    
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("שגיאה מול VirtualBox:");
        eprintln!("{}", stderr);
    } 
}

pub fn clone_vm(base_vm: &str, new_vm_name: &str) -> bool {
    println!("מתחיל לשכפל את מכונת הבסיס '{}' ל-'{}'...", base_vm, new_vm_name);


    let output = Command::new("vboxmanage")
        .args(["clonevm", base_vm, "--name", new_vm_name, "--register"])
        .output()
        .expect("קרתה שגיאה בניסיון להריץ פקודת מערכת לשכפול");

    
    if output.status.success(){
        println!("✅ המכונה '{}' נוצרה בהצלחה!", new_vm_name);
        true

    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("❌ שגיאה בשכפול המכונה:");
        eprintln!("{}", stderr);
        false
    }
}

pub fn start_vm(vm_name: &str, host_ssh_port: u16) -> bool {
    println!("מגדיר הפניית פורט ({} -> 22) עבור המכונה '{}'...", host_ssh_port, vm_name);

    let rule = format!("guestssh,tcp,,{},,22", host_ssh_port);
    let port_fw_output = Command::new("vboxmanage")
        .args(["modifyvm", vm_name, "--natpf1", &rule])
        .output()
        .expect("שגיאה בהגדרת הפניית פורטים");

    if !port_fw_output.status.success(){
        eprintln!("❌ נכשל בהגדרת הפורט ל-SSH");
        return false;
    }
    println!("מדליק את המכונה '{}' ברקע (Headless)...", vm_name);
    
    let start_output = Command::new("vboxmanage")
        .args(["startvm", vm_name, "--type", "headless"])
        .output()
        .expect("שגיאה בניסיון להדליק את המכונה");
    
    if start_output.status.success() {
        println!("✅ המכונה פועלת! אפשר להתחבר אליה דרך localhost:{}", host_ssh_port);
        true
    } else {
        let stderr = String::from_utf8_lossy(&start_output.stderr);
        eprintln!("❌ שגיאה בהדלקת המכונה:");
        eprintln!("{}", stderr);
        false
    }

}

pub fn delete_vm(vm_name: &str) {
    
    println!("מתחיל תהליך מחיקה למכונה '{}'...", vm_name);

    let _ = Command::new("vboxmanage")
        .args(["controlvm", vm_name, "poweroff"])
        .output();

    let delete_output = Command::new("vboxmanage")
        .args(["unregistervm", vm_name, "--delete"])
        .output()
        .expect("שגיאה בניסיון למחוק את המכונה");

    if delete_output.status.success(){
        println!("🗑️ המכונה '{}' נמחקה לחלוטין מהשרת.", vm_name);
    } else {
        let stderr = String::from_utf8_lossy(&delete_output.stderr);
        eprintln!("❌ שגיאה במחיקת המכונה. ייתכן שיהיה צורך למחוק ידנית.");
        eprintln!("{}", stderr);
    }
}


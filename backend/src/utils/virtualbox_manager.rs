use std::process::Command;

pub fn list_existing_vms(){

    println!("-----------------------Trying to communicate with VirtualBox...");

    let output = Command::new("vboxmanage")
        .arg("list")
        .arg("vms")
        .output()
        .expect("------------------------------A fatal error occurred while attempting to run a system command");

    if output.status.success(){
     
        let stdout = String::from_utf8_lossy(&output.stdout);
     
        println!("----------=== Virtual machines exist in VirtualBox ===-----------");
     
        if stdout.trim().is_empty() {
            println!("-----------------------No virtual machines found.");
        } else {
            println!("{}", stdout);
        }

    println!("===============================================");
    
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("---------------------Error with VirtualBox:");
        eprintln!("{}", stderr);
    } 
}

pub fn clone_vm(base_vm: &str, new_vm_name: &str) -> bool {
    println!("-------------------------Starting to clone the base machine '{}' to-'{}'...", base_vm, new_vm_name);


    let output = Command::new("vboxmanage")
        .args(["clonevm", base_vm, "--name", new_vm_name, "--register"])
        .output()
        .expect("-----------------------An error occurred while attempting to run a system command to clone");

    
    if output.status.success(){
        println!("------------------------- ✅ The machine '{}' was successfully created!", new_vm_name);
        true

    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("----------------- ❌ Error duplicating the machine:");
        eprintln!("{}", stderr);
        false
    }
}

pub fn start_vm(vm_name: &str, host_ssh_port: u16) -> bool {
    println!("--------------------------Setting up port forwarding ({} -> 22) for machine '{}'...", host_ssh_port, vm_name);

    let rule = format!("guestssh,tcp,,{},,22", host_ssh_port);
    let port_fw_output = Command::new("vboxmanage")
        .args(["modifyvm", vm_name, "--natpf1", &rule])
        .output()
        .expect("------------------------Error setting up port forwarding");

    if !port_fw_output.status.success(){
        eprintln!("----------------- ❌ Failed to set the port for SSH");
        return false;
    }
    println!("--------------------------Turning on the machine '{}' in the background (Headless)...", vm_name);
    
    let start_output = Command::new("vboxmanage")
        .args(["startvm", vm_name, "--type", "headless"])
        .output()
        .expect("-------------------Error while trying to turn on the machine");
    
    if start_output.status.success() {
        println!("------------------------------- ✅ The machine is running! You can connect to it via localhost:{}", host_ssh_port);
        true
    } else {
        let stderr = String::from_utf8_lossy(&start_output.stderr);
        eprintln!("----------------------- ❌ Error turning on the machine:");
        eprintln!("{}", stderr);
        false
    }

}

pub fn delete_vm(vm_name: &str) {
    
    println!("----------------------Starting deletion process for machine '{}'...", vm_name);

    let _ = Command::new("vboxmanage")
        .args(["controlvm", vm_name, "poweroff"])
        .output();

    let delete_output = Command::new("vboxmanage")
        .args(["unregistervm", vm_name, "--delete"])
        .output()
        .expect("--------------------------Error trying to delete the machine");

    if delete_output.status.success(){
        println!("----------------------- 🗑️ The machine '{}' has been completely deleted from the server.", vm_name);
    } else {
        let stderr = String::from_utf8_lossy(&delete_output.stderr);
        eprintln!("--------------------------- ❌ Error deleting machine. Manual deletion may be necessary.");
        eprintln!("{}", stderr);
    }
}


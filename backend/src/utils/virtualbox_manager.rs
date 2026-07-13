use std::{
    net::{TcpStream, ToSocketAddrs},
    thread,
    time::{Duration, Instant},
};

use std::process::{Command, Output};

const VBOX_MANAGE: &str = "VBoxManage";

fn run_vbox_command(args: &[&str]) -> Result<Output, String> {
    Command::new(VBOX_MANAGE)
        .args(args)
        .output()
        .map_err(|error| {
            format!(
                "Failed to execute '{} {}': {}",
                VBOX_MANAGE,
                args.join(" "),
                error
            )
        })
}

fn output_error(action: &str, output: &Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if !stderr.is_empty() {
        format!("{}: {}", action, stderr)
    } else if !stdout.is_empty() {
        format!("{}: {}", action, stdout)
    } else {
        format!("{} failed without an error message", action)
    }
}

pub fn list_existing_vms() -> Result<Vec<String>, String> {
    println!("[VirtualBox] Retrieving registered virtual machines...");

    let output = run_vbox_command(&["list", "vms"])?;

    if !output.status.success() {
        return Err(output_error(
            "Failed to retrieve registered virtual machines",
            &output,
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let virtual_machines = stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    if virtual_machines.is_empty() {
        println!("[VirtualBox] No registered virtual machines found.");
    } else {
        println!(
            "[VirtualBox] Found {} registered virtual machine(s).",
            virtual_machines.len()
        );

        for virtual_machine in &virtual_machines {
            println!("[VirtualBox] {}", virtual_machine);
        }
    }

    Ok(virtual_machines)
}

pub fn vm_exists(vm_name: &str) -> Result<bool, String> {
    let output = run_vbox_command(&["showvminfo", vm_name])?;

    if output.status.success() {
        return Ok(true);
    }

    let stderr = String::from_utf8_lossy(&output.stderr);

    if stderr.contains("Could not find a registered machine")
        || stderr.contains("Could not find a registered machine named")
    {
        return Ok(false);
    }

    Err(output_error(
        &format!("Failed to check whether VM '{}' exists", vm_name),
        &output,
    ))
}

pub fn clone_vm(base_vm: &str, new_vm_name: &str) -> Result<(), String> {
    println!(
        "[VirtualBox] Cloning template '{}' as '{}'...",
        base_vm, new_vm_name
    );

    if vm_exists(new_vm_name)? {
        return Err(format!(
            "A virtual machine named '{}' already exists",
            new_vm_name
        ));
    }

    if !vm_exists(base_vm)? {
        return Err(format!(
            "The VirtualBox template '{}' does not exist",
            base_vm
        ));
    }

    let output = run_vbox_command(&["clonevm", base_vm, "--name", new_vm_name, "--register"])?;

    if !output.status.success() {
        return Err(output_error(
            &format!(
                "Failed to clone template '{}' as '{}'",
                base_vm, new_vm_name
            ),
            &output,
        ));
    }

    println!("[VirtualBox] VM '{}' was cloned successfully.", new_vm_name);

    Ok(())
}

pub fn configure_ssh_port_forwarding(vm_name: &str, host_ssh_port: u16) -> Result<(), String> {
    println!(
        "[VirtualBox] Configuring SSH forwarding localhost:{} -> guest:22 for '{}'...",
        host_ssh_port, vm_name
    );

    /*
        natpf1 format:

        rule-name,protocol,host-ip,host-port,guest-ip,guest-port
    */
    let forwarding_rule = format!("guestssh,tcp,127.0.0.1,{},,22", host_ssh_port);

    let output = run_vbox_command(&["modifyvm", vm_name, "--natpf1", &forwarding_rule])?;

    if !output.status.success() {
        return Err(output_error(
            &format!("Failed to configure SSH port forwarding for '{}'", vm_name),
            &output,
        ));
    }

    println!(
        "[VirtualBox] SSH port forwarding was configured for '{}'.",
        vm_name
    );

    Ok(())
}

pub fn start_vm(vm_name: &str, host_ssh_port: u16) -> Result<(), String> {
    configure_ssh_port_forwarding(vm_name, host_ssh_port)?;

    println!("[VirtualBox] Starting VM '{}' in headless mode...", vm_name);

    let output = run_vbox_command(&["startvm", vm_name, "--type", "headless"])?;

    if !output.status.success() {
        return Err(output_error(
            &format!("Failed to start virtual machine '{}'", vm_name),
            &output,
        ));
    }

    println!(
        "[VirtualBox] VM '{}' is running. SSH endpoint: 127.0.0.1:{}",
        vm_name, host_ssh_port
    );

    Ok(())
}

pub fn power_off_vm(vm_name: &str) -> Result<(), String> {
    println!("[VirtualBox] Powering off VM '{}'...", vm_name);

    let output = run_vbox_command(&["controlvm", vm_name, "poweroff"])?;

    if output.status.success() {
        println!("[VirtualBox] VM '{}' was powered off.", vm_name);

        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);

    /*
        If the machine is already stopped, deletion can continue.
    */
    if stderr.contains("is not currently running") || stderr.contains("Invalid machine state") {
        println!("[VirtualBox] VM '{}' was already stopped.", vm_name);

        return Ok(());
    }

    Err(output_error(
        &format!("Failed to power off VM '{}'", vm_name),
        &output,
    ))
}

pub fn delete_vm(vm_name: &str) -> Result<(), String> {
    println!("[VirtualBox] Starting deletion of VM '{}'...", vm_name);

    if !vm_exists(vm_name)? {
        println!(
            "[VirtualBox] VM '{}' does not exist. Nothing to delete.",
            vm_name
        );

        return Ok(());
    }

    power_off_vm(vm_name)?;

    let output = run_vbox_command(&["unregistervm", vm_name, "--delete"])?;

    if !output.status.success() {
        return Err(output_error(
            &format!("Failed to unregister and delete VM '{}'", vm_name),
            &output,
        ));
    }

    println!("[VirtualBox] VM '{}' was completely deleted.", vm_name);

    Ok(())
}

pub fn wait_for_ssh(host_ssh_port: u16, timeout: Duration) -> Result<(), String> {
    let address = format!("127.0.0.1:{}", host_ssh_port);

    let socket_address = address
        .to_socket_addrs()
        .map_err(|error| format!("Failed to resolve SSH address '{}': {}", address, error))?
        .next()
        .ok_or_else(|| format!("No socket address found for '{}'", address))?;

    let started_at = Instant::now();

    while started_at.elapsed() < timeout {
        if TcpStream::connect_timeout(&socket_address, Duration::from_secs(2)).is_ok() {
            println!("[VirtualBox] SSH is ready on '{}'.", address);

            return Ok(());
        }

        thread::sleep(Duration::from_secs(2));
    }

    Err(format!(
        "SSH did not become available on '{}' within {} seconds",
        address,
        timeout.as_secs()
    ))
}

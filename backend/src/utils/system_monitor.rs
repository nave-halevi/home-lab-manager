use sysinfo::System;

pub fn check_host_resources() {
    let mut sys = System::new_all();

    sys.refresh_memory();

    let total_ram_gb = sys.total_memory() as f64 / 1_073_741_824.0;
    let used_ram_gb = sys.used_memory() as f64 / 1_073_741_824.0;
    let free_ram_gb = total_ram_gb - used_ram_gb;

    sys.refresh_cpu_usage();
    let core_count = sys.cpus().len();

    println!("=== Host Resource Status ===");
    println!("Total RAM: {:.2} GB", total_ram_gb);
    println!("Used RAM: {:.2} GB", used_ram_gb);
    println!(
        "Free RAM Available for Virtual Machines: {:.2} GB",
        free_ram_gb
    );
    println!("CPU Core Count: {}", core_count);
    println!("============================");
}

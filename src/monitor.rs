use sysinfo::{System, SystemExt};

/**
 * Collects system metrics and returns them as a string.
 * Used for looking at related root cause
 *
 * # Arguments
 * `system` - A reference to a `sysinfo::System` instance.
 * 
 * # Returns
 * A string containing the collected metrics.
 */
pub async fn collect_metrics(system: &System) -> String {
    let mut output = String::new();
    
    for (i, cpu) in system.cpus().iter().enumerate() {
        if let Some(temp) = cpu.temperature() {
            output.push_str(&format!("CPU {}: {}°C\n", i, temp));
        }
    }

    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    let mem_usage = format!(
        "Memory: {} MB used, {} MB free, {} MB total\n",
        used_memory,
        total_memory - used_memory,
        total_memory,
    );
    output.push_str(&mem_usage);

    for disk in system.disks() {
        output.push_str(&format!(
            "Disk {}: {} GB used, {} GB free, {} GB total\n",
            disk.name().to_string_lossy(),
            disk.available_space() / 1024 / 1024 / 1024,
            disk.total_space() / 1024 / 1024 / 1024,
        ));
    }

    output
}

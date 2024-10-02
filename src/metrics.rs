use sysinfo::System;
use sysinfo::Components;
use sysinfo::Disks;

/**
 * Collects system metrics and returns them as a string.
 * Used for looking at related root cause.
 *
 * # Arguments
 * `system` - A reference to a `sysinfo::System` instance.
 * 
 * # Returns
 * A string containing the collected metrics.
 */
pub async fn collect_metrics(system: &System) -> String {
    let mut output = String::new();
    let components = Components::new_with_refreshed_list();
    let disks = Disks::new_with_refreshed_list();
    
    // Collect CPU temperatures
    for comp in &components {
        if let temp = comp.temperature() {
            output.push_str(&format!("{}°C\n", temp));
        }
    }

    // Collect memory usage
    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    let mem_usage = format!(
        "Memory: {} MB used, {} MB free, {} MB total\n",
        used_memory,
        total_memory - used_memory,
        total_memory,
    );
    output.push_str(&mem_usage);

    // Collect disk usage
    for disk in disks.list() {
        output.push_str(&format!(
            "Disk {}: {} GB used, {} GB free, {} GB total\n",
            disk.name().to_string_lossy(),
            (disk.total_space() - disk.available_space()) / 1024 / 1024 / 1024, // Used space
            disk.available_space() / 1024 / 1024 / 1024, // Free space
            disk.total_space() / 1024 / 1024 / 1024 // Total space
        ));
    }

    output
}

mod config;
mod logger;
mod metrics;
mod alerts;
mod summary;

use std::{thread, time};
use std::sync::{Arc, Mutex};
use config::Config;
use logger::Logger;
use metrics::collect_metrics;
use sysinfo::System;
use sysinfo::Components;
use sysinfo::Disks;
use tokio::task;
use summary::Summary;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;

#[tokio::main]
async fn main() {
    let config = Config::load("config.toml");
    let logger = Logger::new(&config.log_file);
    let interval = time::Duration::from_secs(config.check_interval);
    let summary_interval = time::Duration::from_secs(3600); // Summary every hour
    
    let mut system = System::new_all();
    let mut summary = Summary::default();

    let components = Components::new_with_refreshed_list();
    let disks = Disks::new_with_refreshed_list();

    loop {

        let logger_clone = logger.clone();

        // Update system information
        system.refresh_all();

        // Spawn a new task for collecting metrics
        let metrics_output = collect_metrics(&system).await;
        logger_clone.log(&metrics_output).await;

        // Check alerts and gather summary data
        for comp in &components {
            if let temp = comp.temperature() {
                summary.add_cpu_temp(temp);
                if let Some(alert) = alerts::check_alerts(temp, config.temperature_threshold) {
                    logger.log(&alert).await;
                }
            }
        }

        // Collect memory usage
        let used_mem = system.used_memory();
        summary.add_memory_usage(used_mem);

        // Collect disk usage
        for disk in disks.list() {
            summary.add_disk_usage(
                disk.name().to_string_lossy().to_string(),
                disk.total_space() - disk.available_space(),
                disk.total_space(),
            );
        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        // Log summary every hour
        if summary.should_log(current_time) {  // Adjust based on your summary timing logic
            let summary_report = summary.generate_report();
            logger.log_summary(&summary_report).await;
        }

        thread::sleep(interval);
    }
}

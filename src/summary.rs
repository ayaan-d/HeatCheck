use std::collections::HashMap;

#[derive(Default)]
pub struct Summary {
    pub cpu_temps: Vec<f32>,
    pub memory_used: u64,
    pub disk_usage: HashMap<String, (u64, u64)>, // (used, total)
    pub log_interval: u64, // e.g., log every 3600 seconds
    pub last_logged: u64,   // timestamp of the last log
}

impl Summary {
    pub fn add_cpu_temp(&mut self, temp: f32) {
        self.cpu_temps.push(temp);
    }

    pub fn add_memory_usage(&mut self, used: u64) {
        self.memory_used = used;
    }

    pub fn add_disk_usage(&mut self, disk_name: String, used: u64, total: u64) {
        self.disk_usage.insert(disk_name, (used, total));
    }

    pub fn generate_report(&self) -> String {
        let avg_temp = if !self.cpu_temps.is_empty() {
            self.cpu_temps.iter().copied().sum::<f32>() / self.cpu_temps.len() as f32
        } else {
            0.0
        };

        let mut report = format!("Average CPU Temperature: {:.2}°C\n", avg_temp);
        report.push_str(&format!("Total Memory Used: {} MB\n", self.memory_used / 1024 / 1024));

        for (name, (used, total)) in &self.disk_usage {
            report.push_str(&format!(
                "Disk {}: {} GB used, {} GB total\n",
                name,
                used / 1024 / 1024 / 1024,
                total / 1024 / 1024 / 1024,
            ));
        }

        report
    }

    pub fn should_log(&self, current_time: u64) -> bool {
        current_time - self.last_logged >= self.log_interval
    }
}

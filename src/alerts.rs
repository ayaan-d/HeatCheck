pub fn check_alerts(temp: f32, threshold: f32) -> Option<String> {
    if temp > threshold {
        Some(format!("ALERT: Temperature exceeded threshold: {}°C", temp))
    } else {
        None
    }
}

use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

#[derive(Deserialize, Debug)]
struct SensorData {
    raw_value: f64,
    filtered_value: f64,
    anomaly: bool,
    latency_ms: f64,
    jitter_ms: f64,
}

fn main() -> std::io::Result<()> {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let log_filename = format!("RTS_StudentB_Log_{}.txt", timestamp);
    let mut log_file = OpenOptions::new().create(true).append(true).open(&log_filename)?;

    let stream = TcpStream::connect("127.0.0.1:8888")?;
    let reader = BufReader::new(stream);

    writeln!(log_file, "Student B Actuator Log - {}", timestamp)?;
    writeln!(log_file, "Time\tFiltered\tAction\tLatency(ms)")?;

    let mut last_error = 0.0;
    let mut integral = 0.0;

    for line in reader.lines() {
        let line = line?;
        let start = Instant::now();

        let data: SensorData = match serde_json::from_str(&line) {
            Ok(d) => d,
            Err(_) => continue,
        };

        // Simple PID controller logic
        let set_point = 50.0;
        let error = set_point - data.filtered_value;
        integral += error;
        let derivative = error - last_error;
        last_error = error;

        let kp = 1.0;
        let ki = 0.01;
        let kd = 0.1;

        let adjustment = kp * error + ki * integral + kd * derivative;
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;

        writeln!(
            log_file,
            "{:.3}\t{:.2}\t{:.2}\t{:.2}",
            chrono::Local::now().timestamp_millis() as f64,
            data.filtered_value,
            adjustment,
            elapsed
        )?;
    }

    Ok(())
}

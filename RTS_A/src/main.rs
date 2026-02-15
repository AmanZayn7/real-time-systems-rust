use tokio::time::{sleep, Duration, Instant};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use csv::WriterBuilder;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;

const SENSOR_INTERVAL_MS: u64 = 2;
const PROCESSING_DEADLINE_MS: u64 = 2;
const MOVING_AVG_WINDOW: usize = 5;
const ANOMALY_THRESHOLD: f64 = 80.0;

#[derive(Debug, Clone, Serialize)]
struct SensorData {
    raw_value: f64,
    filtered_value: f64,
    anomaly: bool,
    latency_ms: f64,
    jitter_ms: f64,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let file = File::create("sensor_log.csv").expect("Could not create file");
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);
    writer
        .write_record(&["Raw Value", "Filtered Value", "Anomaly", "Latency (ms)", "Jitter (ms)"])
        .expect("Failed to write header");
    let csv_logger = Arc::new(Mutex::new(writer));

    let listener = TcpListener::bind("127.0.0.1:8888").await?;
    println!("[Student A] Waiting for Student B to connect...");

    let (mut socket, _) = listener.accept().await?;
    println!("[Student A] Student B connected!");

    let mut rng = StdRng::from_entropy();
    let mut buffer: Vec<f64> = Vec::with_capacity(MOVING_AVG_WINDOW);
    let mut previous_latency: Option<f64> = None;

    let start_time = Instant::now();

    loop {
        let start = Instant::now();

        let raw_value = rng.gen_range(30.0..=100.0);
        buffer.push(raw_value);
        if buffer.len() > MOVING_AVG_WINDOW {
            buffer.remove(0);
        }

        let filtered_value: f64 = buffer.iter().sum::<f64>() / buffer.len() as f64;
        let is_anomaly = filtered_value > ANOMALY_THRESHOLD;

        sleep(Duration::from_millis(PROCESSING_DEADLINE_MS)).await;

        let latency = start.elapsed().as_secs_f64() * 1000.0;
        let jitter = if let Some(prev) = previous_latency {
            (latency - prev).abs()
        } else {
            0.0
        };
        previous_latency = Some(latency);

        let data = SensorData {
            raw_value,
            filtered_value,
            anomaly: is_anomaly,
            latency_ms: latency,
            jitter_ms: jitter,
        };

        {
            let mut writer = csv_logger.lock().unwrap();
            writer
                .write_record(&[
                    format!("{:.2}", data.raw_value),
                    format!("{:.2}", data.filtered_value),
                    data.anomaly.to_string(),
                    format!("{:.3}", data.latency_ms),
                    format!("{:.3}", data.jitter_ms),
                ])
                .expect("Failed to write to CSV");
            writer.flush().expect("Failed to flush CSV");
        }

        // Send to Student B
        let json = serde_json::to_string(&data).unwrap() + "\n";
        if let Err(e) = socket.write_all(json.as_bytes()).await {
            eprintln!("[Student A] Failed to send data: {}", e);
            break;
        }

        let elapsed = start.elapsed().as_millis() as u64;
        if elapsed < SENSOR_INTERVAL_MS {
            sleep(Duration::from_millis(SENSOR_INTERVAL_MS - elapsed)).await;
        }

        if start_time.elapsed() > Duration::from_secs(10) {
            println!("[Student A] Simulation complete.");
            break;
        }
    }

    Ok(())
}
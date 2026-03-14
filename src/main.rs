use std::convert::Infallible;
use sysinfo::{Disks, System};
use warp::Filter;

fn bytes_to_gb(bytes: u64) -> f64 {
    (bytes as f64) / 1_000_000_000.0
}

async fn metrics_handler() -> Result<impl warp::Reply, Infallible> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_info().cpu_usage();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();

    let total_memory_gb = bytes_to_gb(total_memory);
    let used_memory_gb = bytes_to_gb(used_memory);

    let disks = Disks::new_with_refreshed_list();

    let mut disk_total: u64 = 0;
    let mut disk_available: u64 = 0;

    for disk in &disks {
        disk_total += disk.total_space();
        disk_available += disk.available_space();
    }

    let disk_used = disk_total - disk_available;

    let disk_total_gb = bytes_to_gb(disk_total);
    let disk_used_gb = bytes_to_gb(disk_used);

    let metrics = format!(
        "\
Linux Monitoring Agent Metrics

CPU
cpu_usage_percent: {:.2} %

Memory
memory_used_gb: {:.2} GB
memory_total_gb: {:.2} GB

Disk
disk_used_gb: {:.2} GB
disk_total_gb: {:.2} GB
",
        cpu_usage,
        used_memory_gb,
        total_memory_gb,
        disk_used_gb,
        disk_total_gb
    );

    Ok(metrics)
}

#[tokio::main]
async fn main() {
    let metrics = warp::path("metrics")
        .and(warp::get())
        .and_then(metrics_handler);

    println!("Agent running");
    println!("http://localhost:9100/metrics");

    warp::serve(metrics)
        .run(([0, 0, 0, 0], 9100))
        .await;
}
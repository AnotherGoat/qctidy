use std::{fs, path::PathBuf, process::Command};

use serde_json::json;
use sysinfo::System;

fn main() {
    let system = System::new_all();

    let (cpu_vendor, cpu_brand, cpu_frequency_mhz) = match system.cpus().first() {
        Some(cpu) => (
            cpu.vendor_id().to_string(),
            cpu.brand().to_string(),
            cpu.frequency(),
        ),
        None => (String::new(), String::new(), 0),
    };

    let info = json!({
        "os": System::long_os_version().unwrap_or_default(),
        "os_name": System::name().unwrap_or_default(),
        "os_version": System::os_version().unwrap_or_default(),
        "kernel": System::kernel_version().unwrap_or_default(),
        "hostname": System::host_name().unwrap_or_default(),
        "architecture": System::cpu_arch(),
        "rustc": command_output("rustc", &["--version"]).unwrap_or_default(),
        "cpu_vendor": cpu_vendor,
        "cpu_brand": cpu_brand,
        "cpu_frequency_mhz": cpu_frequency_mhz,
        "physical_cores": System::physical_core_count().unwrap_or(0),
        "logical_cores": system.cpus().len(),
        "total_memory_gib": to_gib(system.total_memory()),
        "total_swap_gib": to_gib(system.total_swap()),
    });

    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("results");
    fs::create_dir_all(&output_dir).unwrap();

    let output_path = output_dir.join("system_info.json");
    let contents = serde_json::to_string_pretty(&info).unwrap();
    fs::write(&output_path, contents).unwrap();

    println!("System information written to {}", output_path.display());
}

fn to_gib(bytes: u64) -> f64 {
    bytes as f64 / 1024.0_f64.powi(3)
}

fn command_output(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;

    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::{
    Components, Disks, Networks, System,
    RefreshKind, CpuRefreshKind
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct SystemData{
    status: String,
    total_memory: u64,
    used_memory: u64,
    cpu_count: usize,
    cpu_average_usage: f32,
    disk_data: Vec<DiskData>
}

#[derive(Serialize, Deserialize, Debug)]
struct DiskData{
    disk_id: i32,
    disk_name: String,
    disk_mount_point: String,
    disk_is_removable: bool,
    disk_total_space: u64,
    disk_available_space: u64
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn get_system_data() -> String {
    println!("System Data");
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());
    
    sys.refresh_cpu(); // Refreshing CPU information.
    let mut total_cpu_usage: f32 = 0.0;

    let mut s = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
    );

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    // Refresh CPUs again.
    s.refresh_cpu();

    for cpu in s.cpus() {
        println!("{}% ", cpu.cpu_usage());
        total_cpu_usage += cpu.cpu_usage();
    }

    total_cpu_usage = total_cpu_usage/sys.cpus().len() as f32;

    let mut disk_data_vec: Vec<DiskData> = Vec::new();

    let disks = Disks::new_with_refreshed_list();
    let mut disk_count = 0;
    for disk in disks.list(){
        println!("[{:?}] {:?}", disk.name(), disk.kind());
        println!("{} {} {} {}", disk.mount_point().display(), disk.total_space(), disk.available_space(), disk.is_removable());

        let disk_data_test = DiskData{
            disk_id: disk_count,
            disk_name: String::from(disk.name().to_str().expect("REASON1").to_string()),
            disk_mount_point: String::from(disk.mount_point().to_str().expect("REASON").to_string()),
            disk_is_removable: disk.is_removable(),
            disk_total_space: disk.total_space(),
            disk_available_space: disk.available_space()
        };
        disk_data_vec.push(disk_data_test);
        disk_count += 1;
    }

    let response = SystemData{
        status: String::from("Success!"),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        cpu_count: sys.cpus().len(),
        cpu_average_usage: total_cpu_usage,
        disk_data: disk_data_vec
    };

    let json_response = serde_json::to_string(&response).unwrap();

    return json_response;
}

fn main() {



    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_system_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use crate::types::PeekError;
use crate::utils::cpu::CpuInfo;
use crate::utils::disks::DisksInfo;
use crate::utils::gpus::vulkan::VulkanInfo;

mod types;
mod utils;

#[tauri::command]
fn get_cpu_info() -> CpuInfo {
    CpuInfo::get()
}

#[tauri::command]
fn get_disks_info() -> DisksInfo {
    DisksInfo::get()
}

#[tauri::command]
fn get_vulkan_info() -> Result<VulkanInfo, PeekError> {
    VulkanInfo::get()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default().plugin(tauri_plugin_shell::init());
    builder
        .invoke_handler(tauri::generate_handler![
            get_cpu_info,
            get_disks_info,
            get_vulkan_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

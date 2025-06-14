use sysinfo::{CpuExt, DiskExt, System, SystemExt};

pub struct SystemInfo {
    system: System,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        SystemInfo { system }
    }
    pub fn get_cpu_usage(&self) -> f32 {
        self.system.global_cpu_info().cpu_usage()
    }
    pub fn get_memory_usage(&self) -> (u64, u64) {
        (self.system.used_memory(), self.system.total_memory())
    }
    pub fn get_disk_usage(&self) -> Vec<(String, u64, u64)> {
        self.system
            .disks()
            .iter()
            .map(|disk| {
                (
                    disk.name().to_str().unwrap().to_string(),
                    disk.total_space(),
                    disk.available_space(),
                )
            })
            .collect()
    }
}

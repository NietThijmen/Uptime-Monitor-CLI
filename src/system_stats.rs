use sysinfo::{
    System, Disks,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Disk {
    pub(crate) name: String,
    pub(crate) total_space: u64,
    pub(crate) used_space: u64,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SysUsage {
    pub(crate) total_cpu: f32,
    pub(crate) used_cpu: f32,

    pub(crate) total_memory: u64,
    pub(crate) used_memory: u64,

    pub(crate) disks: Vec<Disk>,
}

pub(crate)  fn sys_usage() -> SysUsage {
    let mut sys = System::new_all();
    let mut cpu_usage = 0.0;
    let mut cpu_count = 0.0;
    sys.refresh_all();
    sys.refresh_cpu_usage();
    for cpu in sys.cpus() {
        cpu_usage += cpu.cpu_usage();
        cpu_count += 100.0;
    }

    let mut disk_vector = Vec::new();
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        disk_vector.push(Disk {
            name: disk.name().to_string_lossy().into_owned(),
            total_space: disk.total_space(),
            used_space: disk.total_space() - disk.available_space(),
        })
    }

    SysUsage {
        total_cpu: cpu_count,
        used_cpu: cpu_usage,
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        disks: disk_vector,
    }
}
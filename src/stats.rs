// Helper script to get system statistics
use bytesize::ByteSize;
use std::time::Duration;
use sysinfo::{Disks, System};
use systemstat::Platform;
use systemstat::System as systemstat_system;

#[derive(Debug)]
pub struct Distro {
    pub name: String,
    // pub colour: String,
}
#[derive(Debug)]
pub struct MachineInfo {
    pub kernel: String,
    pub arch: String,
    pub node_name: String,
}
#[derive(Debug)]
pub struct Shell {
    pub shell: String,
}
#[derive(Debug)]
pub struct SysInfo {
    pub uptime: Duration,
}

#[derive(Debug)]
pub struct StorageInfo {
    pub total_size: ByteSize,
    pub free: ByteSize,
    pub used: f64,
}

#[derive(Debug)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    // pub free: String
}

#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub remaining_capacity: f32,
    pub remaining_time: u64,
}

pub fn get_distro() -> Option<Distro> {
    let name = System::name();

    Some(Distro { name: name? })
}

pub fn get_machine_info() -> MachineInfo {
    MachineInfo {
        kernel: System::kernel_version().expect("Unable to get kernel information"),
        arch: System::cpu_arch().expect("Unable to get system architecture"),
        node_name: System::host_name().expect("Unable to get system architecture"),
    }
}

pub fn get_shell() -> Shell {
    let system = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::new()),
    );
    let my_pid = sysinfo::get_current_pid().expect("unable to get PID of the current process");
    let parent_pid = system
        .process(my_pid)
        .expect("no self process?")
        .parent()
        .expect("unable to get parent process");
    let parent_process = system
        .process(parent_pid)
        .expect("unable to get parent process");
    let parent_name = parent_process.name();

    Shell {
        shell: parent_name.to_string(),
    }
}

pub fn get_system_information() -> SysInfo {
    SysInfo {
        uptime: Duration::from_secs(System::uptime()),
    }
}

pub fn get_storage_information() -> Result<StorageInfo, Box<dyn std::error::Error>> {
    let sys = systemstat_system::new();

    match sys.mount_at("/") {
        Ok(mount) => {
            //? https://github.com/valpackett/systemstat/blob/trunk/examples/info.rs
            // println!("{} ---{}---> {} (available {} of {})",
            //          mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on, mount.avail, mount.total);
            let total_size: f64 = mount
                .total
                .to_string()
                .split_whitespace()
                .next()
                .expect("Invalid Storage!")
                .parse()
                .unwrap();
            let free: f64 = mount
                .avail
                .to_string()
                .split_whitespace()
                .next()
                .expect("Invalid Storage!")
                .parse()
                .unwrap();

            Ok(StorageInfo {
                total_size: mount.total,
                free: mount.avail,
                used: total_size - free,
            })
        }
        Err(e) => {
            let disks: Disks = Disks::new_with_refreshed_list();
            if let Some(first_disk) = disks.get(0) {
                let total_size: u64 = first_disk.total_space();
                let free: u64 = first_disk.available_space();

                return Ok(StorageInfo {
                    total_size: bytesize::ByteSize(total_size),
                    free: bytesize::ByteSize(free),
                    used: (total_size as f64) - (free as f64),
                });
            } else {
                println!("No disks found.");
                return Err(Box::new(e));
            }
        }
    }
}

pub fn get_memory() -> MemoryInfo {
    let mut sys = System::new_all();

    sys.refresh_all();

    MemoryInfo {
        total: sys.total_memory(),
        used: sys.used_memory(),
    }
}

pub fn get_battery_info() -> Result<BatteryInfo, Box<dyn std::error::Error>> {
    let sys = systemstat_system::new();

    match sys.battery_life() {
        Ok(battery) => Ok(BatteryInfo {
            remaining_capacity: battery.clone().remaining_capacity * 100.0,
            remaining_time: battery.remaining_time.as_secs(),
        }),
        Err(x) => Err(Box::new(x)),
    }
}

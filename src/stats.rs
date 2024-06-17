// Helper script to get system statistics
use bytesize::ByteSize;
use std::collections::HashMap;
use std::fs;
use std::time::Duration;
use sysinfo::{System}; // Components, Disks, Networks
use systemstat::Platform;
use systemstat::System as systemstat_system;

#[derive(Debug)]
pub struct Distro {
    pub name: String,
    pub colour: String,
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
// T is for generic datatype
#[derive(Debug)]
pub struct StorageInfo {
    pub total_size: ByteSize,
    pub free: ByteSize,
    pub used: f64,
}

pub fn get_distro() -> Option<Distro> {
    let os_release = fs::read_to_string("/etc/os-release").ok()?;
    let os_release: HashMap<String, String> = os_release
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|elm| {
            let data = elm
                .split('=')
                .map(|s| s.to_string().replace('\"', ""))
                .collect::<Vec<String>>();
            (data[0].clone(), data[1].clone())
        })
        .collect();

    Some(Distro {
        name: os_release["PRETTY_NAME"].clone(),
        colour: os_release["ANSI_COLOR"].clone(),
    })
}

pub fn get_machine_info() -> MachineInfo {
    //* For some reason the following code has some import issues
    // let info = nix::sys::utsname::UtsName::uname();
    // MachineInfo {
    //     kernel: String::from(info.release()),
    //     arch: String::from(info.machine()),
    //     nodeName: String::from(info.nodename())
    // }
    // let mut sys = System::new_all();
    // First we update all information of our `System` struct.
    // sys.refresh_all();

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
            let total_size: f64 = mount.total.to_string().split_whitespace().next().expect("Invalid Storage!").parse().unwrap();
            let free: f64 = mount.avail.to_string().split_whitespace().next().expect("Invalid Storage!").parse().unwrap();
            
            Ok(StorageInfo {
                total_size: mount.total,
                free: mount.avail,
                used: total_size - free,
            })
        } 
        Err(x) => Err(Box::new(x)),
    }
}

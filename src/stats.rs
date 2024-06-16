// Helper script to get system statistics
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub struct Distro {
    pub name: String,
    pub colour: String,
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

use crate::stats::*;
use colored::*;
use rand::seq::SliceRandom;
use std::time::Duration;

pub struct Colour(pub String);

//* Print banner with random colours
pub fn print_banner() {
    let banner = r#"                                                                                         
    ▄▄▄▄▄                                          ▄▄▄▄▄▄▄▄                                ▄▄       
    ▀▀▀██                        ██                ██▀▀▀▀▀▀              ██                ██       
       ██  ██    ██  ▄▄█████▄  ███████             ██         ▄████▄   ███████    ▄█████▄  ██▄████▄ 
       ██  ██    ██  ██▄▄▄▄ ▀    ██                ███████   ██▄▄▄▄██    ██      ██▀    ▀  ██▀   ██ 
       ██  ██    ██   ▀▀▀▀██▄    ██                ██        ██▀▀▀▀▀▀    ██      ██        ██    ██ 
 █▄▄▄▄▄██  ██▄▄▄███  █▄▄▄▄▄██    ██▄▄▄             ██        ▀██▄▄▄▄█    ██▄▄▄   ▀██▄▄▄▄█  ██    ██ 
  ▀▀▀▀▀     ▀▀▀▀ ▀▀   ▀▀▀▀▀▀      ▀▀▀▀             ▀▀          ▀▀▀▀▀      ▀▀▀▀     ▀▀▀▀▀   ▀▀    ▀▀ 
    "#;

    let colours = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];

    let mut rng = rand::thread_rng();
    let random_color = colours.choose(&mut rng).unwrap();

    println!("{}", banner.color(*random_color));
}

pub fn print_distro_into() {
    if let Some(distro) = get_distro() {
        println!(
            "{}\t\t{}",
            "❯ OS".blue(),
            distro.name.truecolor(255, 102, 102).bold()
        );
    } else {
        println!("Could not read distro information");
    }
}

pub fn print_machine_info(machine_info: MachineInfo) {
    println!(
        "{}\t{}",
        "❯ Kernel".truecolor(0, 255, 136),
        machine_info.kernel.purple().bold()
    );
    println!(
        "{}\t{}",
        "❯ Architecture".cyan(),
        machine_info.arch.bright_blue().bold()
    );
    println!(
        "{}\t{}",
        "❯ Hostname".yellow(),
        machine_info.node_name.truecolor(205, 133, 63).bold()
    );
}

pub fn print_shell_info(shell: String) {
    println!(
        "{}\t\t{}",
        "❯ Shell".purple(),
        shell.truecolor(0, 166, 136).bold()
    );
}

pub fn print_uptime(duration: Duration) {
    let hours = (duration.as_secs() / 3600) as u32;
    let minutes = ((duration.as_secs() % 3600) / 60) as u32;
    let time: String = format!("{:02}h {:02}m", hours, minutes);

    println!("{}\t{}", "❯ Uptime".truecolor(255, 0, 212), time.bold());
}

pub fn print_storage_info(storage_info: StorageInfo) {
    let total_size = storage_info.total_size.to_string();
    let free = storage_info.free.to_string();
    let _used = storage_info.used;

    println!(
        "{}\t\t{} / {}",
        "❯ Disk".truecolor(51, 255, 119),
        free.bold().truecolor(153, 255, 102),
        total_size.bold().truecolor(153, 255, 102)
    );
}

pub fn print_ram_info(memory_info: MemoryInfo) {
    let ram_used: f64 = memory_info.used as f64 / 1_000_000_000.0;
    let total_used: f64 = memory_info.total as f64 / 1_000_000_000.0;

    println!(
        "{}\t{:.3} GB / {:.2} GB",
        "❯ Memory".truecolor(77, 225, 255),
        ram_used.to_string().bold().truecolor(255, 153, 204),
        total_used.to_string().bold().truecolor(255, 153, 204)
    );
}

pub fn print_battery_info() {
    let battert_info = get_battery_info();
    let _ = battert_info.as_ref().unwrap().remaining_time;

    println!(
        "{}\t{}{}",
        "❯ Battery".truecolor(255, 179, 191),
        battert_info
            .as_ref()
            .unwrap()
            .remaining_capacity
            .to_string()
            .bold()
            .truecolor(179, 255, 255),
        "%".truecolor(178, 255, 255).bold() // (battert_info.as_ref().unwrap().remaining_time / 3600).to_string().truecolor(255, 153, 204),
                                            // (battert_info.as_ref().unwrap().remaining_time % 60).to_string().truecolor(255, 153, 204)
    );
}

pub fn color_scheme() -> Vec<Colour> {
    (40..=47)
        .chain(100..=107)
        .map(|cl| Colour(format!("\x1B[{}m", cl)))
        .collect::<Vec<Colour>>()
}

pub fn print_pallets() {
    let mut clrs = String::from("");
    let colours = color_scheme();
    for (indx, color) in colours.into_iter().enumerate() {
        if indx == 16 {
            clrs += "\n"
        }
        clrs += &format!("{}  \x1B[0m", color.0);
    }

    clrs += "\n";
    println!("{}", clrs)
}

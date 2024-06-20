use crate::stats::*;
use colored::*;
use rand::seq::SliceRandom;
use std::time::Duration;

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

    let colors = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];

    let mut rng = rand::thread_rng();
    let random_color = colors.choose(&mut rng).unwrap();

    println!("{}", banner.color(*random_color));
}

// Function to convert ANSI color code to colored::Color
pub fn ansi_to_colored_string(text: &str, ansi_code: &str) -> ColoredString {
    let codes: Vec<&str> = ansi_code.split(';').collect();
    if codes.len() == 2 {
        let intensity = codes[0];
        let color_code = codes[1];

        let color = match color_code {
            "30" => Color::Black,
            "31" => Color::Red,
            "32" => Color::Green,
            "33" => Color::Yellow,
            "34" => Color::Blue,
            "35" => Color::Magenta,
            "36" => Color::Cyan,
            "37" => Color::White,
            _ => return text.normal(),
        };

        if intensity == "1" {
            text.color(color).bold()
        } else {
            text.color(color)
        }
    } else {
        text.normal()
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

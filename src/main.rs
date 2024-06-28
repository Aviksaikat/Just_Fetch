/* TODOs
OS name [*]
kernel version [*]
shell [*]
uptime [*]
disk usage [*]
memory use [*]
window manager
packages
*/
mod print;
mod stats;

use clap::Parser;
use std::error::Error;
use std::time::Duration;

//* Custom Creates
use print::*;
use stats::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, action)]
    no_banner: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();

    if !cli.no_banner {
        print_banner();
    }

    print_distro_into();

    let machine_info = get_machine_info();
    print_machine_info(machine_info);

    let shell_name: String = get_shell().shell;
    print_shell_info(shell_name);

    let duration: Duration = get_system_information().uptime;
    print_uptime(duration);

    let storage_info: StorageInfo = get_storage_information()?;
    print_storage_info(storage_info);

    let memory_info: MemoryInfo = get_memory();
    print_ram_info(memory_info);

    print_battery_info();

    println!();
    print_pallets();

    Ok(())
}

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

use colored::Colorize;
use std::error::Error;
use std::time::Duration;

//* Custom Creates
use print::*;
use stats::*;


fn main() -> Result<(), Box<dyn Error>> {
    print_banner();
    if let Some(distro) = get_distro() {
        let colored_name = ansi_to_colored_string(&distro.name, &distro.colour);
        println!("{}\t\t{}", "❯ OS".blue(), colored_name);
    } else {
        println!("Could not read distro information");
    }

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

    Ok(())
}

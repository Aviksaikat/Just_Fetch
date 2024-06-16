/* TODOs
OS name
kernel version
shell
uptime
disk usage
memory use
window manager
packages
*/
mod print;
mod stats;

use colored::Colorize;
use std::error::Error;

//* Custom Creates
use print::{ansi_to_colored_string, print_banner};
use stats::get_distro;

// fn main() {
//     let distro_name: String = stats::get_distro().unwrap().name;
//     let distro_colour: String = stats::get_distro().unwrap().colour;

//     // Convert the ANSI color code to colored::Color
//     let coloured_name = ansi_to_colored_string(&distro_name, &distro_colour);
//     println!("OS:\t{}", coloured_name);
// }

fn main() -> Result<(), Box<dyn Error>> {
    print_banner();
    if let Some(distro) = get_distro() {
        let colored_name = ansi_to_colored_string(&distro.name, &distro.colour);
        println!("{}\t{}", "OS".blue(), colored_name);
    } else {
        println!("Could not read distro information");
    }

    Ok(())
}

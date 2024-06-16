use colored::*;
use rand::seq::SliceRandom;

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

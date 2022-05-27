mod color_utils;

// eventually use: https://crates.io/crates/anyhow/1.0.57
use ansi_term::{ANSIString, ANSIStrings, Colour::RGB, Style};
use regex::Regex;
use std::{io, u8};

fn main() {
    let colors_re = Regex::new(r"#?([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})").unwrap();
    let color_re = Regex::new(r"^#?[0-9a-fA-F]{6}$").unwrap();
    println!("Input a color");
    let mut user_color = String::new();
    io::stdin()
        .read_line(&mut user_color)
        .expect("Failed to get input");

    // remove newline
    user_color.pop();
    if !color_re.is_match(&user_color) {
        println!("{}", RGB(250, 90, 80).bold().paint("Invalid Color"));
        return;
    }
    let rgb_red = u8::from_str_radix(
        &colors_re
            .captures(&&user_color)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str(),
        16,
    )
    .unwrap();

    let rgb_green = u8::from_str_radix(
        &colors_re
            .captures(&&user_color)
            .unwrap()
            .get(2)
            .unwrap()
            .as_str(),
        16,
    )
    .unwrap();
    let rgb_blue = u8::from_str_radix(
        &colors_re
            .captures(&&user_color)
            .unwrap()
            .get(3)
            .unwrap()
            .as_str(),
        16,
    )
    .unwrap();

    let color_tup = (rgb_red, rgb_green, rgb_blue);

    let strings: &[ANSIString<'static>] = &[
        Style::new().bold().paint("Your color is:\n"),
        RGB(color_tup.0, color_tup.1, color_tup.2)
            .paint(format!("#{:02x}{:02x}{:02x}", rgb_red, rgb_green, rgb_blue)),
    ];

    println!("{}", ANSIStrings(strings));

    println!("[l]ighten or [d]arken the color:");
    let mut action = String::new();
    io::stdin()
        .read_line(&mut action)
        .expect("Failed to get input");

    action.pop();
    if action != "l" && action != "d" {
        println!("{}", RGB(250, 90, 80).bold().paint("Invalid Action"));
    }

    let new_colors: (u8, u8, u8);
    let mut amount = String::new();
    println!("Input the amount to use (0-255):");
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to get input");
    let amount_int: u8 = amount
        .trim()
        .parse()
        .expect("Not a number between 0 and 255");

    if action == "l" {
        new_colors = color_utils::adjusting::lighten_color(&color_tup, amount_int);
    } else {
        new_colors = color_utils::adjusting::darken_color(&color_tup, amount_int);
    }

    let light_strings: &[ANSIString<'static>] = &[
        Style::new().bold().paint("Your adjusted color up is:\n"),
        RGB(new_colors.0, new_colors.1, new_colors.2).paint(format!(
            "#{:02x}{:02x}{:02x}",
            new_colors.0, new_colors.1, new_colors.2
        )),
    ];
    println!("{}", ANSIStrings(light_strings));
}

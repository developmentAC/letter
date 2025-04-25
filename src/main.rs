// Begin by importing necessary crates and modules
use clap::{Arg, Command}; // For parsing command-line arguments
use colored::Colorize;
use std::collections::HashMap; // For storing letter data in a map
use std::fs; // For file and directory operations
use std::io::Write;
use std::path::{Path, PathBuf}; // For working with file paths // For flushing output to the console

use std::io;

mod toml_extract; // Extract and print the version information according to the toml file

// Function to display the banner
fn show_banner() {
    // banner ref: https://manytools.org/hacker-tools/ascii-banner/

    //logo design: "ticks", use "█" to replace "/\" chars, "_" replaced with space
    let banner = String::from(
        "
\t   @@╗        @@@@@@@╗   @@@@@@@@╗   @@@@@@@@╗   @@@@@@@╗   @@@@@@╗    
\t   ██╗        ███████╗   ████████╗   ████████╗   ███████╗   ██████╗    
\t   ██║        ██╔════╝   ╚══██╔══╝   ╚══██╔══╝   ██╔════╝   ██╔══██╗   
\t   ██║        █████╗        ██║         ██║      █████╗     ██████╔╝   
\t   ██║        ██╔══╝        ██║         ██║      ██╔══╝     ██╔══██╗   
\t   ███████╗   ███████╗      ██║         ██║      ███████╗   ██║  ██║   
\t   ╚══════╝   ╚══════╝      ╚═╝         ╚═╝      ╚══════╝   ╚═╝  ╚═╝ 
\t    ╚══════╝   ╚══════╝      ╚═╝         ╚═╝      ╚══════╝   ╚═╝  ╚═╝ 
\t     ╚══════╝   ╚══════╝      ╚═╝         ╚═╝      ╚══════╝   ╚═╝  ╚═╝ 

",
    );

    // Print the banner in purple color
    colour_print(&banner, "cyan")
}

// Print colored text to the console
fn colour_print(text: &str, colour: &str) {
    match colour {
        "flush_green" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            io::stdout().flush().unwrap();
            print!(" {}", text.bright_green().bold());
            io::stdout().flush().unwrap();
        }
        "green" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_green().bold());
        }
        "green_noLineFeed" => {
            print!("\x1b[2K\r");
            print!("{}", text.bright_green().bold());
        }
        "red" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_red().bold());
        }
        "cyan" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_cyan().bold());
        }
        "purple" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_purple().bold());
        }
        "purple_noLineFeed" => {
            print!("\x1b[2K\r");
            print!("{}", text.bright_purple().bold());
        }
        "blue" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_blue().bold());
        }
        "yellow" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_yellow().bold());
        }
        "yellow_noLineFeed" => {
            print!("\x1b[2K\r");
            print!("{}", text.bright_yellow().bold());
        }
        _ => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_yellow().bold());
        }
    }
}

fn main() {
    // Show the banner
    show_banner();

    // Display version information from the toml file
    toml_extract::main();

    // Print a welcome message and quick help instruction
    let msg = format!(
        "Welcome to Letter, an ASCII letter writer!\n\t Use: \"cargo run -- --bighelp\" for quick help."
    );
    println!("\t {}", msg.bright_yellow().bold());
    println!(
        "\t {}",
        "-------------------------------------------------\n"
            .bright_yellow()
            .bold()
    );

    // Step 1: Set up command-line argument parsing
    let matches = Command::new("Letter Maker")
        .version("1.0")
        .author("Oliver Bonham-Carter <obonhamcarter@allegheny.edu>")
        .about("Generates large ASCII art for input strings")
        .arg(
            Arg::new("input")
                .help("The string to convert to ASCII art")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("bighelp")
                .short('b')
                .long("bighelp")
                .action(clap::ArgAction::SetTrue)
                .help("Get a sample prompt to send to the model."),
        )
        .arg(
            Arg::new("output")
                .help("The output filename")
                .short('o')
                .long("output")
                // .takes_value(true)
                .required(false)
                .default_value("output.txt"),
        )
        .get_matches();

    // Retrieve the Big Help message flag
    let big_help = matches.get_flag("bighelp");

    // If big_help is requested, display the help message and exit
    if big_help {
        get_big_help();
        return;
    }

    // Step 2: Get the input string from the command-line arguments
    let input = matches.get_one::<String>("input").unwrap().to_uppercase();

    // Step 3: Get the output filename
    let output_filename = matches.get_one::<String>("output").unwrap();

    // Step 4: Define the large character data
    let letter_data = get_letter_data();

    // Step 5: Generate the output
    let mut output_lines = vec!["".to_string(); 6];
    for ch in input.chars() {
        if let Some(letter) = letter_data.get(&ch) {
            for (i, line) in letter.iter().enumerate() {
                output_lines[i].push_str(line);
                output_lines[i].push(' ');
            }
        } else {
            eprintln!("Warning: Character '{}' not found in letter data.", ch);
        }
    }
    // add a line at the end of the output
    output_lines.push('\n'.to_string());

    // Step 6: Ensure the output directory exists
    ensure_output_directory_exists("0_out");

    // Step 7: Get a unique filename if the file already exists
    let output_path = get_unique_filename("0_out", output_filename);

    // Step 8: Save the output to the file
    if let Err(e) = fs::write(&output_path, output_lines.join("\n")) {
        eprintln!("Error writing to file {}: {}", output_path.display(), e);
    } else {
        // Print the output to the console
        let msg = format!(
            "\t + Your output has been saved to: {}",
            output_path.display()
        );
        colour_print(&msg, "cyan");

        println!(
            "\n{}\n",
            "\t + The output is the following:".bright_cyan().bold()
        )

        // println!("Output saved to {}", output_path.display());
    }

    for line in output_lines {
        println!("{}", line.bright_cyan().bold());
    }
}

// Print out the help message
fn get_big_help() {
    let msg = format!("\n\t cargo run -- --output letter.md letter")
        .bright_cyan()
        .bold();
    println!("{}", msg);
}

// Function to ensure the output directory exists
fn ensure_output_directory_exists(dir: &str) {
    let path = Path::new(dir);
    if !path.exists() {
        if let Err(e) = fs::create_dir(path) {
            eprintln!("Error creating directory {}: {}", dir, e);
        }
    }
}

// Function to get a unique filename if the file already exists
fn get_unique_filename(dir: &str, filename: &str) -> PathBuf {
    let mut path = PathBuf::from(dir).join(filename);
    let mut counter = 1;

    while path.exists() {
        let file_stem = Path::new(filename)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        let extension = Path::new(filename)
            .extension()
            .map_or("".to_string(), |ext| format!(".{}", ext.to_string_lossy()));

        let new_filename = format!("{}{}{}", file_stem, counter, extension);
        path = PathBuf::from(dir).join(new_filename);
        counter += 1;
    }

    path
}

// Function to define the ASCII art data for each letter
fn get_letter_data() -> HashMap<char, Vec<&'static str>> {
    let mut map = HashMap::new();
    map.insert(
        'A',
        vec![
            "    ███╗   ",
            "   ████╗   ",
            "  ██╔██╗   ",
            " ██╔╝██╗   ",
            "███████╗   ",
            "╚══════╝   ",
        ],
    );
    map.insert(
        'B',
        vec![
            "██████╗   ",
            "██╔══██╗  ",
            "██████╔╝  ",
            "██╔═══╝   ",
            "██████╗   ",
            "╚═════╝   ",
        ],
    );
    map.insert(
        'C',
        vec![
            " ██████╗  ",
            "██╔════╝  ",
            "██║       ",
            "██║       ",
            "╚██████╗  ",
            " ╚═════╝  ",
        ],
    );
    map.insert(
        'D',
        vec![
            "██████╗   ",
            "██╔══██╗  ",
            "██║  ██║  ",
            "██║  ██║  ",
            "██████╔╝  ",
            "╚═════╝   ",
        ],
    );
    map.insert(
        'E',
        vec![
            "███████╗  ",
            "██╔════╝  ",
            "█████╗    ",
            "██╔══╝    ",
            "███████╗  ",
            "╚══════╝  ",
        ],
    );
    map.insert(
        'F',
        vec![
            "███████╗  ",
            "██╔════╝  ",
            "█████╗    ",
            "██╔══╝    ",
            "██║       ",
            "╚═╝       ",
        ],
    );
    map.insert(
        'G',
        vec![
            " ██████╗   ",
            "██╔════╝   ",
            "██║  ███╗  ",
            "██║   ██║  ",
            "╚██████╔╝  ",
            " ╚═════╝   ",
        ],
    );
    map.insert(
        'H',
        vec![
            "██╗  ██╗  ",
            "██║  ██║  ",
            "███████║  ",
            "██╔══██║  ",
            "██║  ██║  ",
            "╚═╝  ╚═╝  ",
        ],
    );
    map.insert(
        'I',
        vec![
            "██╗  ",
            "██║  ",
            "██║  ",
            "██║  ",
            "██║  ",
            "╚═╝  "],
    );
    map.insert(
        'J',
        vec![
            "     ██╗  ",
            "     ██║  ",
            "     ██║  ",
            "██   ██║  ",
            "╚█████╔╝  ",
            " ╚════╝   ",
        ],
    );
    map.insert(
        'K',
        vec![
            "██╗  ██╗  ",
            "██║ ██╔╝  ",
            "█████╔╝   ",
            "██╔═██╗   ",
            "██║  ██╗  ",
            "╚═╝  ╚═╝  ",
        ],
    );
    map.insert(
        'L',
        vec![
            "██╗       ",
            "██║       ",
            "██║       ",
            "██║       ",
            "███████╗  ",
            "╚══════╝  ",
        ],
    );
    map.insert(
        'M',
        vec![
            "███╗   ███╗  ",
            "████╗ ████║  ",
            "██╔████╔██║  ",
            "██║╚██╔╝██║  ",
            "██║ ╚═╝ ██║  ",
            "╚═╝     ╚═╝  ",
        ],
    );
    map.insert(
        'N',
        vec![
            "███╗   ██╗  ",
            "████╗  ██║  ",
            "██╔██╗ ██║  ",
            "██║╚██╗██║  ",
            "██║ ╚████║  ",
            "╚═╝  ╚═══╝  ",
        ],
    );
    map.insert(
        'O',
        vec![
            " ██████╗   ",
            "██╔═══██╗  ",
            "██║   ██║  ",
            "██║   ██║  ",
            "╚██████╔╝  ",
            " ╚═════╝   ",
        ],
    );
    map.insert(
        'P',
        vec![
            "██████╗   ",
            "██╔══██╗  ",
            "██████╔╝  ",
            "██╔═══╝   ",
            "██║       ",
            "╚═╝       ",
        ],
    );
    map.insert(
        'Q',
        vec![
            " ██████╗   ",
            "██╔═══██╗  ",
            "██║   ██║  ",
            "██║▄▄ ██║  ",
            "╚██████╔╝  ",
            " ╚══▀▀═╝   ",
        ],
    );
    map.insert(
        'R',
        vec![
            "██████╗   ",
            "██╔══██╗  ",
            "██████╔╝  ",
            "██╔══██╗  ",
            "██║  ██║  ",
            "╚═╝  ╚═╝  ",
        ],
    );
    map.insert(
        'S',
        vec![
            "███████╗  ",
            "██╔════╝  ",
            "███████╗  ",
            "╚════██║  ",
            "███████║  ",
            "╚══════╝  ",
        ],
    );
    map.insert(
        'T',
        vec![
            "████████╗  ",
            "╚══██╔══╝  ",
            "   ██║     ",
            "   ██║     ",
            "   ██║     ",
            "   ╚═╝     ",
        ],
    );
    map.insert(
        'U',
        vec![
            "██╗   ██╗  ",
            "██║   ██║  ",
            "██║   ██║  ",
            "██║   ██║  ",
            "╚██████╔╝  ",
            " ╚═════╝   ",
        ],
    );
    map.insert(
        'V',
        vec![
            " ██╗   ██╗  ",
            " ██║   ██║  ",
            " ██║   ██║  ",
            " ╚██╗ ██╔╝  ",
            "  ╚████╔╝   ",
            "   ╚═══╝    ",
        ],
    );
    map.insert(
        'W',
        vec![
            "██╗    ██╗  ",
            "██║    ██║  ",
            "██║ █╗ ██║  ",
            "██║███╗██║  ",
            "╚███╔███╔╝  ",
            " ╚══╝╚══╝   ",
        ],
    );
    map.insert(
        'X',
        vec![
            "██╗  ██╗  ",
            "╚██╗██╔╝  ",
            " ╚███╔╝   ",
            " ██╔██╗   ",
            "██╔╝ ██╗  ",
            "╚═╝  ╚═╝  ",
        ],
    );
    map.insert(
        'Y',
        vec![
            "██╗   ██╗ ",
            "╚██╗ ██╔╝ ",
            " ╚████╔╝  ",
            "  ╚██╔╝   ",
            "   ██║    ",
            "   ╚═╝    ",
        ],
    );
    map.insert(
        'Z',
        vec![
            "███████╗  ",
            "╚══███╔╝  ",
            "  ███╔╝   ",
            " ███╔╝    ",
            "███████╗  ",
            "╚══════╝  ",
        ],
    );
    map.insert(
        '!',
        vec![
            " ██╗  ",
            " ██║  ",
            " ██║  ",
            "      ",
            " ██║  ",
            " ╚═╝  "],
    );
    map.insert(
        '.',
        vec![
            "     ",
            "     ",
            "     ",
            "     ",
            " ██║  ",
            " ╚═╝ "],
    );
    map.insert(
        '@',
        vec![
            " ██████╗   ",
            "██╔═══██╗  ",
            "██║██╗██║  ",
            "██║██║██║  ",
            "╚█║████╔╝  ",
            " ╚╝╚═══╝   ",
        ],
    );
    map.insert(
        '_',
        vec![
            "          ",
            "          ",
            "          ",
            "          ",
            "███████╗  ",
            "╚══════╝  ",
        ],
    );
    map.insert(
        '-',
        vec![
            "          ",
            "          ",
            "███████╗  ",
            "╚══════╝  ",
            "          ",
            "          ",
        ],
    );
    map.insert(
        //comma
        ':',
        vec![
            "      ",
            "███╗  ",
            "╚══╝  ",
            "███╗  ",
            "╚══╝  ",
            "      "
            ],
    );

    map.insert(
        //colon
        '\'',
        vec![
            "  ██╗ ",
            " ██╔╝ ",
            "██╔╝  ",
            "╚═╝   ",
            "      ",
            "      "],
    );
    map
}

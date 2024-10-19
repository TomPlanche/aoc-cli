///
/// # src/main.rs
/// Contains the main function for the program.
///
/// ## Author
/// Tom Planche <github.com/tomPlanche>
///
// Imports  ==============================================================================  Imports
use chrono::Datelike;
use clap::{Parser, Subcommand};
use toml_edit::{ArrayOfTables, DocumentMut, Table};

use std::{fs, path::PathBuf, process::Command};

mod file_utils;
use crate::file_utils::{
    check_file_struct_integrity_year, check_global_file_struct_integrity, init_folders_and_files,
};

// Variables  =========================================================================== Variables
#[derive(Parser)]
#[command(
    about = "Custom binary that can generate folders, files, download input files for the \
Advent of Code challenges."
)]
#[command(author = "Tom P. <tomplanche@icloud.com>")]
#[command(help_template = "{about}\nMade by: {author}\n\nUSAGE:\n{usage}\n\n{all-args}\n")]
#[command(name = "aoc")]
struct Cli {
    /// Commands
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Init subcommand
#[derive(Subcommand)]
#[command(
    about = "Creates all the folders needed for the Advent of Code challenges of the given year."
)]
enum Commands {
    /// Init subcommand
    /// Creates all the folders and files needed for the Advent of Code challenges of the given year.
    /// The year is 2015 by default.
    Init {
        /// The year of the Advent of Code challenge
        #[arg(short, long)]
        year: Option<u16>,
    },

    /// Create subcommand
    /// Creates all needed files for the given day of the given year.
    /// The year is by default the one in the .cargo/config.toml file.
    /// The day is the current day by default.
    Create {
        /// The day of the Advent of Code challenge
        #[arg(short, long, value_parser = parse_day, default_value = env!("AOC_DAY"))]
        day: Option<u8>,

        /// The year of the Advent of Code challenge
        #[arg(short, long, value_parser = parse_year, default_value = env!("AOC_YEAR"))]
        year: Option<u16>,
    },

    /// Solve subcommand
    /// Changes the 'src/main.rs' file to run the tests of the given day.
    /// Compiles and runs the tests.
    Solve {
        /// The day of the Advent of Code challenge
        #[arg(short, long, value_parser = parse_day, default_value = env!("AOC_DAY"))]
        day: Option<u8>,

        /// The year of the Advent of Code challenge
        #[arg(short, long, value_parser = parse_year, default_value = env!("AOC_YEAR"))]
        year: Option<u16>,

        /// Problem part 2 argument
        /// If passed, the 'src/main.rs' file will be changed to run both parts of the given day.
        #[arg(short, long, default_value = "false")]
        part_2: bool,
    },
}
// Functions  =========================================================================== Functions
///
/// # parse_day
/// Parses the day argument.
/// The day should be between 1 and 25.
///
/// ## Arguments
/// * `s` - The string to parse
///
/// ## Returns
/// * `Result<u8, std::num::ParseIntError>` - The parsed day
fn parse_day(s: &str) -> Result<u8, String> {
    let day = s.parse::<u8>().unwrap();
    if day < 1 || day > 25 {
        return Err("The day should be between 1 and 25.".parse().unwrap());
    }
    Ok(day)
}

///
/// # parse_year
/// Parses the year argument.
/// The day should be between 2015 and the current year.
///
/// ## Arguments
/// * `s` - The string to parse
///
/// ## Returns
/// * `Result<u16, std::num::ParseIntError>` - The parsed year
fn parse_year(s: &str) -> Result<u16, String> {
    let day = s.parse::<u16>().unwrap();
    if day < 2015 || day > get_current_year() {
        return Err("The year should be between 2015 and the current year"
            .parse()
            .unwrap());
    }
    Ok(day)
}

///
/// # get_current_year
/// Returns the current year.
///
/// ## Arguments
/// * `()` - Nothing
///
/// ## Returns
/// * `u16` - The current year
fn get_current_year() -> u16 {
    let now = chrono::Utc::now();

    now.year() as u16
}

///
/// # update_cargo_toml
///
/// Updates the 'Cargo.toml' file by removing the existing [[bin]] sections and maybe adding a new one for the given day and year.
///
/// ## Arguments
///
/// * `caller` - The path of the caller
/// * `day` - The day of the Advent of Code challenge
/// * `year` - The year of the Advent of Code challenge
/// * `add_bin` - If true, a new [[bin]] section will be added for the given day and year
///
/// ## Returns
/// * `Result<(), Box<dyn std::error::Error>>` - The result of the operation
fn update_cargo_toml(
    caller: &PathBuf,
    day: u8,
    year: u16,
    add_bin: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let cargo_toml_path = caller.join("Cargo.toml");
    let toml_content = fs::read_to_string(&cargo_toml_path)?;
    let mut doc = toml_content.parse::<DocumentMut>()?;

    // Remove existing [[bin]] sections
    doc.remove("bin");

    if add_bin {
        let desired_name = format!("day_{:02}_year_{}", day, year);
        let mut bin_array = ArrayOfTables::new();
        let mut bin_table = Table::new();

        bin_table.insert("name", toml_edit::value(desired_name));
        bin_table.insert("path", toml_edit::value("src/main.rs"));
        bin_array.push(bin_table);

        doc["bin"] = toml_edit::Item::ArrayOfTables(bin_array);
    }

    fs::write(&cargo_toml_path, doc.to_string())?;
    Ok(())
}

///
/// # compile_and_run_tests
/// Compiles and runs the tests.
///
/// ## Arguments
/// * `caller` - The folder from which the program was called
/// * `day` - The day of the Advent of Code challenge
/// * `year` - The year of the Advent of Code challenge
///
/// ## Returns
/// * `()` - Nothing
fn compile_solution(caller: &PathBuf, day: u8, year: u16) {
    // Add [[bin]] section
    if let Err(e) = update_cargo_toml(caller, day, year, true) {
        eprintln!("Failed to update Cargo.toml: {}", e);
        return;
    }
    // Run the build command
    let mut build_command = Command::new("cargo");
    build_command
        .current_dir(caller)
        .arg("build")
        .arg("--release");

    // Execute the build command
    let status = build_command
        .status()
        .expect("Failed to execute build command");

    if !status.success() {
        println!("Failed to compile the solution");
        return;
    }

    // Remove [[bin]] section after compilation
    if let Err(e) = update_cargo_toml(caller, day, year, false) {
        eprintln!("Failed to clean up Cargo.toml: {}", e);
    }
}

// Main  ====================================================================================  Main
fn main() {
    // Get the arguments
    let cli = Cli::parse();

    // Folder caller - the folder from which the program was called
    let caller = std::env::current_dir().unwrap();

    match &cli.command {
        Some(Commands::Init { year }) => {
            // If the 'year' argument is passed
            if let Some(year) = year {
                // Create the folders and files
                init_folders_and_files(&caller, *year);
            } else {
                // Create the folders and files
                init_folders_and_files(&caller, env!("AOC_YEAR").parse::<u16>().unwrap());
            }
        }
        Some(Commands::Create { day, year }) => {
            let day = day.unwrap_or(env!("AOC_DAY").parse::<u8>().unwrap());
            let year = year.unwrap_or(env!("AOC_YEAR").parse::<u16>().unwrap());

            if !check_global_file_struct_integrity(&caller)
                || !check_file_struct_integrity_year(&caller, year)
            {
                println!("The file structure is not correct.\nPlease run `cargo aoc init` or `cargo aoc init --year desired_year` to create the folders and files needed for the Advent of Code challenges.");
                return;
            }

            // Create the folders and files
            file_utils::create_files(&caller, day, year);
        }
        Some(Commands::Solve { day, year, part_2 }) => {
            let day = day.unwrap_or(env!("AOC_DAY").parse::<u8>().unwrap());
            let year = year.unwrap_or(env!("AOC_YEAR").parse::<u16>().unwrap());

            if !check_global_file_struct_integrity(&caller)
                || !check_file_struct_integrity_year(&caller, year)
            {
                println!("The file structure is not correct.\nPlease run `cargo aoc init` or `cargo aoc init --year desired_year` to create the folders and files needed for the Advent of Code challenges.");
                return;
            }

            // Prepare the 'src/main.rs' file
            file_utils::prepare_main_file(&caller, day, year, *part_2);

            // Compile the solution
            compile_solution(&caller, day, year);
        }
        None => {
            println!("No command passed");
        }
    }
}

// Tests ==================================================================================== Tests
#[test]
fn test_parse_day() {
    assert_eq!(parse_day("1").unwrap(), 1);
    assert_eq!(parse_day("25").unwrap(), 25);
    assert_eq!(
        parse_day("26").unwrap_err(),
        "The day should be between 1 and 25."
    );
}

#[test]
fn test_parse_year() {
    assert_eq!(parse_year("2015").unwrap(), 2015);
    assert_eq!(parse_year("2023").unwrap(), 2023);
    assert_eq!(
        parse_year("2024").unwrap_err(),
        "The year should be between 2015 and the current year"
    );
}

#[test]
fn test_get_current_year() {
    assert_eq!(get_current_year(), 2023);
}

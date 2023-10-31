///
/// # src/main.rs
/// Contains the main function for the program.
///
/// ## Author
/// Tom Planche <github.com/tomPlanche>
///

// Imports  ==============================================================================  Imports
use clap::{Parser, Subcommand};
use std::{
    fs::create_dir,
    path::Path,
};

use crate::file_utils::create_folder;

mod file_utils;

// Variables  =========================================================================== Variables
#[derive(Parser)]
#[command(about = "Custom binary that can generate folders, files, download input files for the \
Advent of Code challenges.")]
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
#[command(about = "Creates all the folders needed for the Advent of Code challenges of the given year.")]
enum Commands {
    /// Init subcommand
    /// Creates all the folders and files needed for the Advent of Code challenges of the given year.
    /// The year is 2015 by default.
    Init {
        /// The year of the Advent of Code challenge
        #[arg(short, long)]
        year: Option<u16>,
    },
}
// Functions  =========================================================================== Functions
///
/// # create_folders_and_files
/// Creates all the folders and files needed for the Advent of Code challenges of the given year.
/// The year is 2015 by default.
///
/// The folders and files are created in the folder from which the program was called and are
/// structured like this:
/// ```text
/// .
/// ├── src/
/// │   ├── main.rs
/// │   ├── bin/
/// │   │   ├── aoc_cli
/// │   │   ├── download_input.rs
/// │   │   └── solve.rs
/// │   ├── year_2015/
/// │   │   ├── day_01.rs
/// │   │   ├── ...
/// │   │   └── day_25.rs
//  │   └── year_n/
/// ├── data/
/// │   ├── year_2015/
/// │   │   ├──puzzles/
/// │   │   │   ├── day_01.md
/// │   │   │   ├── ...
/// │   │   │   └── day_25.md
/// │   │   └── inputs/
/// │   │       ├── day_01.txt
/// │   │       ├── ...
/// │   │       └── day_25.txt
/// │   └── year_n/
///
/// ## Arguments
/// * `caller` - The folder from which the program was called
/// * `year` - The year of the Advent of Code challenge
///
/// ## Returns
/// * `()` - Nothing
fn init_folders_and_files(caller: &Path, year: u16) {
    println!("Creating folders and files for the year {} @ {}\n 🎄 Happy coding !", year, caller.display());

    // Create the 'src' folder, check if it already exists
    let src_folder = caller.join("src");
    if !src_folder.exists() {
        create_dir(&src_folder).expect("Something went wrong creating the 'src' folder");
    }

    // Create the 'src/bin' folder, check if it already exists
    let src_bin_folder = src_folder.join("bin");
    create_folder(&src_bin_folder);

    // Create the 'year_n' folder, check if it already exists
    let year_folder = src_folder.join(format!("year_{}", year));
    create_folder(&year_folder);

    // Create the 'data' folder, check if it already exists
    let data_folder = caller.join("data");
    create_folder(&data_folder);

    // Create the 'data/year_n' folder, check if it already exists
    let data_year_folder = data_folder.join(format!("year_{}", year));
    create_folder(&data_year_folder);

    // Create the 'data/year_n/puzzles' folder, check if it already exists
    let data_year_puzzles_folder = data_year_folder.join("puzzles");
    create_folder(&data_year_puzzles_folder);

    // Create the 'data/year_n/inputs' folder, check if it already exists
    let data_year_inputs_folder = data_year_folder.join("inputs");
    create_folder(&data_year_inputs_folder);
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
        None => {
            println!("No command passed");
        },
    }
}

///
/// # src/main.rs
/// Contains the main function for the program.
///
/// ## Author
/// Tom Planche <tomplanche@icloud.com>
///

// Imports  ==============================================================================  Imports
use clap::{Parser};
use std::{
    fs::create_dir,
    path::Path,
};


// Variables  =========================================================================== Variables
#[derive(Parser)]
#[command(about = "Custom binary that can generate folders, files, download input files for the \
Advent of Code challenges.")]
#[command(author = "Tom P. <tomplanche@icloud.com>")]
#[command(help_template = "{about}\nMade by: {author}\n\nUSAGE:\n{usage}\n\n{all-args}\n")]
#[command(name = "aoc")]
struct Cli {
    /// Optional 'init' argument.
    /// Creates all the folders and files needed for the Advent of Code challenges.
    #[arg(short, long)]
    init: bool,

    /// Optional 'year' argument.
    /// The year of the Advent of Code challenge.
    #[arg(short, long)]
    year: Option<u16>,
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
    if !src_bin_folder.exists() {
        create_dir(&src_bin_folder).expect("Something went wrong creating the 'src/bin' folder");
    }

    // Create the 'year_n' folder, check if it already exists
    let year_folder = src_folder.join(format!("year_{}", year));
    if !year_folder.exists() {
        create_dir(&year_folder).expect("Something went wrong creating the 'year_n' folder");
    }

    // Create the 'data' folder, check if it already exists
    let data_folder = caller.join("data");
    if !data_folder.exists() {
        create_dir(&data_folder).expect("Something went wrong creating the 'data' folder");
    }

    // Create the 'data/year_n' folder, check if it already exists
    let data_year_folder = data_folder.join(format!("year_{}", year));
    if !data_year_folder.exists() {
        create_dir(&data_year_folder).expect("Something went wrong creating the 'data/year_n' folder");
    }

    // Create the 'data/year_n/puzzles' folder, check if it already exists
    let data_year_puzzles_folder = data_year_folder.join("puzzles");
    if !data_year_puzzles_folder.exists() {
        create_dir(&data_year_puzzles_folder).expect("Something went wrong creating the 'data/year_n/puzzles' folder");
    }

    // Create the 'data/year_n/inputs' folder, check if it already exists
    let data_year_inputs_folder = data_year_folder.join("inputs");
    if !data_year_inputs_folder.exists() {
        create_dir(&data_year_inputs_folder).expect("Something went wrong creating the 'data/year_n/inputs' folder");
    }

}

// Main  ====================================================================================  Main
fn main() {
    // Get the arguments
    let args = Cli::parse();

    // Folder caller - the folder from which the program was called
    let caller = std::env::current_dir().unwrap();

    // If the 'init' argument is passed
    if args.init {
        // If the 'year' argument is passed
        if let Some(year) = args.year {
            // Create the folders and files
            init_folders_and_files(&caller, year);
        } else {
            // Create the folders and files
            init_folders_and_files(&caller, env!("AOC_YEAR").parse::<u16>().unwrap());
        }
    }
}

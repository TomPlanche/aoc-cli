///
/// # src/main.rs
/// Contains the main function for the program.
///
/// ## Author
/// Tom Planche <tomplanche@icloud.com>
///

// Imports  ==============================================================================  Imports
use clap::{Parser};

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
///
/// ## Arguments
/// * `year` - The year of the Advent of Code challenge
///
/// ## Returns
/// * `()` - Nothing
fn create_folders_and_files(year: u16) {
    println!("Creating folders and files for the year {}...", year);
}

// Main  ====================================================================================  Main
fn main() {
    // Get the arguments
    let args = Cli::parse();

    // If the 'init' argument is passed
    if args.init {
        // If the 'year' argument is passed
        if let Some(year) = args.year {
            // Create the folders and files
            create_folders_and_files(year);
        } else {
            // Create the folders and files
            create_folders_and_files(env!("AOC_YEAR").parse::<u16>().unwrap());
        }
    }
}

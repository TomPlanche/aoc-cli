use std::fs::create_dir;
///
/// # file_utils.rs
/// Contains the utils for files.
///
/// The final file structure should look like this:
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
/// └── data/
///     ├── year_2015/
///     │   ├──puzzles/
///     │   │   ├── day_01.md
///     │   │   ├── ...
///     │   │   └── day_25.md
///     │   └── inputs/
///     │       ├── day_01.txt
///     │       ├── ...
///     │       └── day_25.txt
///     └── year_n/
///
/// /// Tom Planche <github.com/tomPlanche>

// Imports  ==============================================================================  Imports
use std::{
    path::Path,
};

// Variables  =========================================================================== Variables
const AOC_URL: &str = "https://adventofcode.com/";

// Functions  =========================================================================== Functions
pub fn check_global_file_struct_integrity(caller: &Path) -> bool {
    // Check if the 'src' folder exists
    let src_folder = caller.join("src");
    if !src_folder.exists() {
        return false;
    }

    // Check if the 'src/bin' folder exists
    let src_bin_folder = src_folder.join("bin");
    if !src_bin_folder.exists() {
        return false;
    }

    // Check if the 'data' folder exists
    let data_folder = caller.join("data");
    if !data_folder.exists() {
        return false;
    }

    true
}

///
/// # check_file_struct_integrity
/// Checks if the file structure is valid.
///
/// ## Arguments
/// * `year` - The year of the Advent of Code challenge
///
/// ## Returns
/// * `bool` - If the file structure is valid or not
pub fn check_file_struct_integrity_year(caller: &Path, year: u16) -> bool {
    // Check if the 'src/year_n' folder exists
    let src_year_folder = caller.join(format!("src/year_{}", year));
    if !src_year_folder.exists() {
        return false;
    }

    // Check if the 'data/year_n' folder exists
    let data_year_folder = caller.join(format!("data/year_{}", year));
    if !data_year_folder.exists() {
        return false;
    }

    // Check if the 'data/year_n/puzzles' folder exists
    let data_year_puzzles_folder = data_year_folder.join("puzzles");
    if !data_year_puzzles_folder.exists() {
        return false;
    }

    // Check if the 'data/year_n/inputs' folder exists
    let data_year_inputs_folder = data_year_folder.join("inputs");
    if !data_year_inputs_folder.exists() {
        return false;
    }

    true
}

///
/// # create_files
/// Creates the files needed for the Advent of Code challenges for the given day and year.
///
/// ## Arguments
/// * `day` - The day of the Advent of Code challenge
/// * `year` - The year of the Advent of Code challenge
///
/// ## Returns
/// * `()` - Nothing
pub fn create_files(caller: &Path, day: u8, year: u16) {
    // Check if the file structure is valid
    if !check_global_file_struct_integrity(caller)
        || !check_file_struct_integrity_year(caller, year) {
        println!("The file structure is not correct.\nPlease run `cargo aoc init` or `cargo aoc init --year desired_year` to create the folders and files needed for the Advent of Code challenges.");
        return;
    }

    // Create the 'src/year_n/day_n.rs' file
    let src_year_day_file = caller.join(format!("src/year_{}/day_{:02}.rs", year, day));
    if !src_year_day_file.exists() {
        std::fs::File::create(&src_year_day_file).expect("Failed to create file !");
    }

    // Prepare the 'data/year_n/puzzles/day_n.md' file
    let data_year_puzzles_day_file = caller.join(format!("data/year_{}/puzzles/day_{:02}.md", year, day));
    // If the file doesn't exist
    if !data_year_puzzles_day_file.exists() {
        // Create it
        std::fs::File::create(&data_year_puzzles_day_file).expect("Failed to create file !");
    }

    // Prepare the 'data/year_n/inputs/day_n.txt' file
    let data_year_inputs_day_file = caller.join(format!("data/year_{}/inputs/day_{:02}.txt", year, day));
    // If the file doesn't exist
    if !data_year_inputs_day_file.exists() {
        // Create it
        std::fs::File::create(&data_year_inputs_day_file).expect("Failed to create file !");
    }

    let message = format!("Files for day {} of year {} created !\nGo to {AOC_URL}{}/day/{} to see the puzzle :).", day, year, year, day);
    println!("{}", message);
}

///
/// # create_folder
/// Creates a folder at the given path.
/// If the folder already exists, does nothing.
///
/// ## Arguments
/// * `path` - The path of the folder to create
///
/// ## Returns
/// * `()` - Nothing
pub fn create_folder(path: &Path) {
    // If the folder doesn't exist
    if !path.exists() {
        // Create it
        create_dir(path).expect("Failed to create folder !");
    }
}

///
/// # create_folders_and_files
/// Creates all the folders and files needed for the Advent of Code challenges of the given year.
/// The year is 2015 by default.
///
/// The folders and files are created in the folder from which the program was called and are
/// structured like the one given in the file doc.
///
/// ## Arguments
/// * `caller` - The folder from which the program was called
/// * `year` - The year of the Advent of Code challenge
///
/// ## Returns
/// * `()` - Nothing
pub fn init_folders_and_files(caller: &Path, year: u16) {
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

/*
 * End of file src/file_utils.rs
 */

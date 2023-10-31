use std::fs::create_dir;
///
/// # file_utils.rs
///
///
/// /// Tom Planche <github.com/tomPlanche>

// Imports  ==============================================================================  Imports
use std::path::Path;

// Variables  =========================================================================== Variables

// Functions  =========================================================================== Functions
pub fn create_folder(path: &Path) {
    // If the folder doesn't exist
    if !path.exists() {
        // Create it
        create_dir(path).expect("Failed to create folder !");
    }
}

/*
 * End of file src/file_utils.rs
 */

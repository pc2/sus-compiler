use std::{fs, path::PathBuf};

fn main() {
    let mut install_dir = get_sus_dir();
    install_dir.push(env!("CARGO_PKG_VERSION"));
    install_dir.push("std");
    
    fs::create_dir_all(&install_dir).expect("Failed to create std_lib directory");

    copy_dir("std", &install_dir).expect("Failed to copy STD folder");

    // Print the path to make it available during the build
    println!("cargo:rustc-env=SUS_COMPILER_STD_LIB_PATH={}", install_dir.display());
}

fn get_sus_dir() -> PathBuf {
    let mut sus_dir = dirs_next::home_dir().expect("Could not determine home directory");
    sus_dir.push(".sus");

    // Create the .sus directory if it doesn't exist
    if !sus_dir.exists() {
        fs::create_dir(&sus_dir).expect("Failed to create .sus directory");
    }

    sus_dir
}

// Helper function to copy a directory and its contents recursively
fn copy_dir(src: &str, dst: &PathBuf) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir(path.to_str().unwrap(), &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

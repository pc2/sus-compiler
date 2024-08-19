use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let install_dir = PathBuf::from(out_dir)
        .join("../../..")
        .join("share")
        .join("sus_compiler")
        .join("std");
    
    fs::create_dir_all(&install_dir).expect("Failed to create std_lib directory");

    copy_dir("stl", &install_dir).expect("Failed to copy STL folder");

    // Print the path to make it available during the build
    println!("cargo:rustc-env=SUS_COMPILER_STD_LIB_PATH={}", install_dir.display());
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

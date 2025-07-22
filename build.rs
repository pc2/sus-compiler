use std::{
    fs,
    path::{Path, PathBuf},
};

fn main() {
    let home_dir = get_sus_dir().join(env!("CARGO_PKG_VERSION"));
    let std_dir = home_dir.join("std");

    fs::create_dir_all(&std_dir).expect("Failed to create std_lib directory");

    copy_dir("std", &std_dir).expect("Failed to copy STD folder");

    // Print the path to make it available during the build
    println!("cargo:rustc-env=SUS_HOME={}", home_dir.to_str().unwrap());

    // note: add error checking yourself.
    let output = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!(
        "cargo:rustc-env=BUILD_DATE={}",
        chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
    );
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
fn copy_dir(src: &str, dst: &Path) -> std::io::Result<()> {
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

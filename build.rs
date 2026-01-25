use std::{
    fs,
    path::{Path, PathBuf},
};

fn main() -> Result<(), String> {
    let home_dir = get_sus_dir()?;
    let std_dir = home_dir.join("std");

    copy_dir(&PathBuf::from("std"), &std_dir);

    // Print the path to make it available during the build
    println!(
        "cargo:rustc-env=INSTALL_SUS_HOME={}",
        home_dir.to_str().unwrap()
    );

    let version_str = std::env::var_os("CARGO_PKG_VERSION").unwrap();
    if version_str.to_str().unwrap().ends_with("-dev") {
        let build_features = if std::env::var_os("CARGO_FEATURE_LSP").is_some() {
            ""
        } else {
            " without LSP Support"
        };
        let git_hash = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap();
        let git_hash = String::from_utf8(git_hash.stdout).unwrap();
        let git_hash = git_hash.trim();
        let build_date = chrono::Local::now().format("%Y-%m-%d_%H:%M:%S");

        println!(
            "cargo:rustc-env=EXTRA_VERSION_STRING= ({git_hash}) built at {build_date}{build_features}"
        );
    } else {
        println!("cargo:rustc-env=EXTRA_VERSION_STRING=");
    }

    Ok(())
}

fn get_sus_dir() -> Result<PathBuf, String> {
    Ok(
        if let Some(sus_install_dir) = std::env::var_os("INSTALL_SUS_HOME") {
            let sus_install_dir = PathBuf::from(sus_install_dir);

            let help_str = "When manually specifying $INSTALL_SUS_HOME you should create it first as an empty directory.";

            let sus_install_dir = sus_install_dir.canonicalize().map_err(|e| {
                format!(
                    "The directory {} does not exist. {help_str} ({e})",
                    sus_install_dir.to_string_lossy()
                )
            })?;
            let Ok(mut dir_iter) = sus_install_dir.read_dir() else {
                return Err(format!(
                    "{} exists but is a file??? {help_str}",
                    sus_install_dir.to_string_lossy()
                ));
            };
            if dir_iter.next().is_some() {
                return Err(format!(
                    "The directory {} is not empty. Does it contain a previous install? {help_str}",
                    sus_install_dir.to_string_lossy()
                ));
            }

            sus_install_dir
        } else {
            let mut sus_dir = dirs::data_dir().expect("Could not determine data directory");
            sus_dir.push("sus");
            sus_dir.push(env!("CARGO_PKG_VERSION"));
            if let Err(e) = fs::create_dir_all(&sus_dir) {
                return Err(format!(
                    "Failed to create {} directory: {e}",
                    sus_dir.to_string_lossy()
                ));
            };

            sus_dir
        },
    )
}

fn copy_dir(src: &Path, dst: &Path) {
    if dst.is_dir() {
        fs::remove_dir_all(dst).unwrap();
    } else if dst.is_file() {
        fs::remove_file(dst).unwrap();
    }
    fs::create_dir(dst).unwrap();

    copy_dir_recurse(src, dst);
}
// Helper function to copy a directory and its contents recursively
fn copy_dir_recurse(src: &Path, dst: &Path) {
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            fs::create_dir(&dest_path).unwrap();
            copy_dir_recurse(&path, &dest_path);
        } else {
            fs::copy(&path, &dest_path).unwrap();
        }
    }
}

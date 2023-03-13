use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [-rf] <file or directory>", args[0]);
        process::exit(1);
    }

    let file = PathBuf::from(&args[args.len() - 1]);
    let is_dir = file.is_dir();

    if args.len() == 3 && args[1] == "-rf" {
        if !is_dir {
            eprintln!("Error: {} is not a directory", file.display());
            process::exit(1);
        }

        // Recursively remove directory contents and move to Trash
        remove_dir_contents(&file).expect("Failed to remove directory contents");

        // Remove the now-empty directory
        fs::remove_dir(&file).expect("Failed to remove directory");

        println!("Directory {} moved to Trash", file.display());
    } else {
        if is_dir {
            eprintln!("Error: {} is a directory. Use the -rf flag to move directories", file.display());
            process::exit(1);
        }

        if !file.exists() {
            eprintln!("Error: {} does not exist", file.display());
            process::exit(1);
        }

        let trash_dir = match env::var_os("HOME") {
            Some(home) => PathBuf::from(home).join(".Trash"),
            None => {
                eprintln!("Error: $HOME environment variable not set");
                process::exit(1);
            }
        };

        if !trash_dir.exists() {
            fs::create_dir(&trash_dir).expect("Failed to create .Trash directory");
        }

        let new_file_path = trash_dir.join(&file.file_name().unwrap());

        fs::rename(&file, &new_file_path).expect("Failed to move file to Trash");

        println!("{} moved to {}", file.display(), new_file_path.display());
    }
}

fn remove_dir_contents(dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_dir() {
            remove_dir_contents(&path)?;
            fs::remove_dir(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}

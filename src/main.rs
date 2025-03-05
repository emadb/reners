use colored::Colorize;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args[1] == "--help" {
        println!(
            "{}",
            "Usage: reners <starting_folder> <pattern_to_search> <replace_value>".yellow()
        );
        println!("More info https://github.com/emadb");
        std::process::exit(1);
    }

    if args.len() != 4 {
        eprintln!(
            "{}",
            "Usage: reners <starting_folder> <pattern_to_search> <replace_value>".red()
        );
        std::process::exit(1);
    }

    let path = args[1].as_str();
    let to_find = args[2].as_str();
    let to_replace = args[3].as_str();

    _ = list_content(path, to_find, to_replace);

    Ok(())
}

fn list_content(starting_path: &str, to_find: &str, to_replace: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(starting_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            try_rename(&path, to_find, to_replace)
        }
        if path.is_dir() {
            _ = list_content(path.to_str().unwrap(), to_find, to_replace);
        }
    }

    Ok(())
}

fn try_rename(path: &Path, to_find: &str, to_replace: &str) {
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        if file_name.contains(to_find) {
            let new_name = file_name.replace(to_find, to_replace);
            let new_path = path.with_file_name(new_name);

            if let Err(e) = fs::rename(path, &new_path) {
                eprintln!("{} {}", "Failed to rename:".red(), e);
            } else {
                println!("Renamed: {:?}", new_path);
            }
        }
    }
}

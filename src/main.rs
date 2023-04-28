mod config;
mod file;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

const SCRIPT_FILENAME: &'static str = ".archie.sh";
const ARCHIVE_DIR: &'static str = "archie";
const ZERO: u8 = 48;

fn main() {
    let args = Args::parse();
    if !args.path.is_dir() {
        panic!("Provided path is ot a direcotry!")
    }
    let script_path = args.path.join(SCRIPT_FILENAME);
    let archive_dir = args.path.join(ARCHIVE_DIR);
    if !script_path.exists() {
        panic!("No script found in directory");
    }
    let paths = fs::read_dir(args.path).unwrap();
    let mut paths_to_archive: Vec<PathBuf> = [].to_vec();
    for path in paths {
        let real_path = path.unwrap();
        let metadata = real_path.metadata().unwrap();
        if metadata.is_dir() || real_path.file_name() == SCRIPT_FILENAME {
            continue;
        }

        let result = Command::new("sh")
            .arg(script_path.as_path())
            .arg(real_path.path())
            .output()
            .expect("damn")
            .stdout;

        if result.len() < 1 {
            panic!("Script output is incorrect");
        }

        if result[0] != ZERO {
            paths_to_archive.push(real_path.path());
        }
    }

    if !archive_dir.exists() {
        fs::create_dir(&archive_dir).unwrap();
    }
    for path in paths_to_archive {
        move_file(path, &archive_dir);
    }
}

fn move_file(file: PathBuf, archive_dir: &PathBuf) {
    let file_name = file.file_name().unwrap();
    let new_file = archive_dir.join(file_name);
    fs::rename(file, new_file).unwrap();
}

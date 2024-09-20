use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target_dir = "."; // Default target directory is current directory
    if args.len() >= 2 {
        target_dir = &args[1];
    }

    let target = path::PathBuf::from(target_dir);
    println!("Target directory: {:?}", target);
    tree(&target, 0);
}

fn tree(target: &path::PathBuf, depth: i32) {
    let files = target.read_dir().expect("Target directory does not exists");
    for ent in files {
        let path = ent.unwrap().path();
        for _ in 0..depth {
            print!("|  ");
        }
        let fname = path.file_name().unwrap().to_string_lossy(); // Convert OsString to String

        if path.is_dir() {
            println!("+-- {}", fname);
            tree(&path, depth + 1); // Recursivly search the directory when it is a directory
            continue;
        }
        println!("|  {}", fname);
    }
}

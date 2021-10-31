use ansi_term::Color;
use clap::App;
use std::process::Command;
use std::{env, fs, str};

fn main() {

    let curr_dir = env::current_dir().unwrap();

    let dir_list = fs::read_dir(curr_dir);

    if let Ok(successful_list) = dir_list {
        
        for dir in successful_list {
            
            if let Ok(successful_entry) = dir {
            
                if let Ok(_) = env::set_current_dir(successful_entry.path()) {
                    get_status(successful_entry);
                }
            
            }
        }

    }
}

fn get_status(dir: fs::DirEntry) {
    let git_status_result = Command::new("git").args(["status", "-s", "-b"]).output();

    if let Ok(status) = git_status_result {
        if status.stderr.is_empty() {
            println!(
                "{}:",
                Color::Cyan
                    .bold()
                    .paint(dir.path().to_str().unwrap())
                    .to_string()
            );

            analyze_result(str::from_utf8(&status.stdout).unwrap().to_string());
        }
    }
}

fn analyze_result(status: String) {
    
    get_branch(&status);

    let split_str = &status
    .split("\n")
    .filter(|&line| line != "")
    .collect::<Vec<&str>>();

    match split_str.len() {
        1 => println!(
            "{}",
            Color::Green
                .bold()
                .paint("  • Nothing to commit")
                .to_string()
        ),
        _ => {
            println!(
                "{}",
                Color::Red
                    .bold()
                    .paint("  • Uncommitted changes:")
                    .to_string()
            );
        }
    }



    get_uncommitted_files(&split_str[1..]);

    println!();

}

fn get_uncommitted_files(split_str: &[&str]) {


    for (index, file) in split_str.iter().enumerate() {
        let indexed_str = format!("\t{}: {}", index, file);

        let colorized_str = if indexed_str.contains(" D ") {
            Color::Red.paint(indexed_str).to_string()
        } else if indexed_str.contains(" M ") {
            Color::Yellow.paint(indexed_str).to_string()
        } else {
            Color::White.paint(indexed_str).to_string()
        };

        println!("{}", colorized_str);
    }
}

fn get_branch(status: &String) {
    let split_str = status.split("\n").collect::<Vec<&str>>();

    let branch = match status.contains("## No commits yet on") {
        true => split_str[0].replace("## No commits yet on ", ""),

        false => split_str[0].replace("## ", ""),
    };

    let split_branch = branch.split(" ").collect::<Vec<&str>>();

    match split_branch.len() {
        3 => println!(
            "  • Branch: {} {}",
            Color::White
                .bold()
                .underline()
                .paint(split_branch[0])
                .to_string(),
            match &split_branch[1..].concat() {
                i if i.contains("behind") => Color::Red.bold().underline().paint(i).to_string(),
                i if i.contains("ahead") => Color::Yellow.bold().underline().paint(i).to_string(),
                i => Color::White.bold().underline().paint(i).to_string(),
            }
        ),

        _ => println!(
            "  • Branch: {}",
            Color::White.bold().paint(split_branch[0]).to_string(),
        ),
    }
}

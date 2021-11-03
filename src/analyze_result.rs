

pub mod analyze_result {

    use ansi_term::Color;
    use std::path::PathBuf;
    use std::process::Command;
    use std::{env, fs, str};

    /*
    this function gets a directory listing, then it tries to change the pwd to
    each directory listing while trying to get the git status
    */
    pub fn print_status_then_list(curr_dir: PathBuf) {

        if !(get_status(curr_dir.canonicalize().unwrap())) {
        
            //get dir list
            let dir_list = fs::read_dir(curr_dir);
                                                /* .unwrap()
                                                    .filter(|&entry| {
                                                        entry.unwrap().metadata().unwrap().is_dir()
                                                    })
                                                    .collect::<Vec<>();*/

            //ensure it can be unwrapped safely
            if let Ok(successful_list) = dir_list {
                
                //filter out files and dot directories (hidden directories)
                let filtered_list = successful_list
                                                    .map(
                                                        |entry|
                                                            entry.unwrap()
                                                    )
                                                    .filter(
                                                        |entry| 
                                                            entry.metadata().unwrap().is_dir()

                                                    )
                                                    .filter(
                                                        |entry| 
                                                        !entry.path().to_str().unwrap().contains("/.")
                                                    )
                                                    .collect::<Vec<fs::DirEntry>>();

                //for each direcotry or file in the unwrapped listing
                for dir in filtered_list {

                    //check that it can be unwrapped safely
                    //if let Ok(successful_entry) = dir {
                            //copy the path just to avoid borrowing shenanigans
                            print_status_then_list(dir.path());
                    //}
                }

            }
        }

    }



    //gets the current status of the git repo, then tries to throw it to the analysis;
    //returns true if a status was successfully gotten, otherwise false
    fn get_status(dir: PathBuf) -> bool{

        if let Ok(_) = env::set_current_dir(&dir) {
        
            //run git command
            let git_status_result = Command::new("git").args(["status", "-s", "-b"]).output();

            //make sure the status can be unwrapped
            if let Ok(status) = git_status_result {
                
                if status.stderr.is_empty() {
                    println!(
                        "{}:",
                        Color::Cyan
                            .bold()
                            .paint(dir.to_str().unwrap())
                            .to_string()
                    );

                    //this function tries to analyze the stdout
                    analyze_result(str::from_utf8(&status.stdout).unwrap().to_string());
                }

                else {

                    return false

                }
            }
        }

        return true
    }

    fn analyze_result(status: String) {
        
        //get the branch name (and if it's ahead/behind)
        get_branch(&status);

        //split up the status so it can analyzed
        let split_str = &status
        .split("\n")
        .filter(|&line| line != "")
        .collect::<Vec<&str>>();

        //if the status is one line, then there is nothing to commit
        //i.e. the only status info is the branch name
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


        //try to get the uncommitted files
        get_uncommitted_files(&split_str[1..]);

        println!();

    }

    fn get_uncommitted_files(split_str: &[&str]) {

        //give each uncommitted file a color;
        //red is for deleted files
        //yellow is for modified files
        //white is for unknown or untracked
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

    //gets the branch
    fn get_branch(status: &String) {
        let split_str = status.split("\n").collect::<Vec<&str>>();

        //two situtations: either a new and yet committed branch
        //or has uncommitted files
        let branch = match status.contains("## No commits yet on") {
            true => split_str[0].replace("## No commits yet on ", ""),

            false => split_str[0].replace("## ", ""),
        };

        let split_branch = branch.split(" ").collect::<Vec<&str>>();


        //if split.branch is len() == 3, then there is a section saying either 
        //[behind #] or [ahead #]
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
                Color::White.bold().underline().paint(split_branch[0]).to_string(),
            )
        }
    }

}

pub use analyze_result::print_status_then_list;
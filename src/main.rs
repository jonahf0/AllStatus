use clap::{Arg, App};
use std::path::PathBuf;
use std::env;
use pager::Pager;
use dirs::home_dir;

mod analyze_result;

/*
the entry point of the program--will hopefully have more to in the future,
but for now it just gets the cwd and then starts the recursion
*/
fn main() {

    //use the clap crate to parse the arguments
    let matches = App::new("all_test").about("\nDesigned to show the status of all of the host's Git repos' statuses")
    .arg(Arg::from_usage("--pager [PAGER] 'pager to try to pipe status info into; default is 'less''"))
    .arg(Arg::from_usage("--root [DIR] 'top-level directory to search for repos from; default is $HOME'"))
    .get_matches();

    let pager  = match matches.value_of("pager") {
                        
                    Some(page) => page,
                    
                    _ => "less -cR"
                    
                };

    let curr_dir = PathBuf::from(
                    
                    match matches.value_of("root") {

                        Some(dir) => PathBuf::from(dir),

                        _ => home_dir()
                            .unwrap_or(env::current_dir().unwrap())
                }
            );

    let _process_pager = Pager::with_default_pager(pager).setup();

    //call the listing function
    analyze_result::print_status_then_list(curr_dir);
}

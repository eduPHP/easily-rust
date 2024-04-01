mod docker;
mod projects;
pub mod composer;
pub mod stubs;
pub mod ssl;
use std::env;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_help();
        return;
    }
    let action = args[1].trim();
    match action {
        "create" => {
            if !composer::exists() {
                println!("Could not guess the project name, please input it as an argument.");
                print_help();
                return;
            }
            let project = projects::try_to_guess();

            projects::create(&project);

        },
        "start" => {
            if args.len() <= 2 {
                if !composer::exists() {
                    println!("Could not guess the project name, please input it as an argument.");
                    print_help();
                    return;
                }
                let project = projects::try_to_guess();
                projects::run(&project);
            } else {
                projects::run(args[2].trim());
            }
        },
        _ => print_help(),
    }
}

fn print_help() {
    println!("Usage: easily [start|stop|create|remove] <project>");
    println!("- start <project>\t Starts the project");
    println!("- stop <?project>\t Stops the given project or the project currently running");
    println!("- restart <?project>\t Restart the given project or the project currently running");
    println!("- create <project>\t Creates a new project");
    println!("- remove <project>\t Removes the project containers from docker");
    println!("- help\t\t\t Shows this help message\n");
}

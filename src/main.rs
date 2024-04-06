mod docker;
mod projects;
pub mod composer;
pub mod stubs;
pub mod ssl;
pub mod msg;
pub mod cmd;
pub mod config;
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
            // personalizado: php, composer, artisan
            projects::create(&projects::try_to_guess());
        },
        "stop" => {
            projects::stop(&projects::try_to_guess());
        },
        "init" => projects::init(),
        "start" => {
            if args.len() <= 2 {
                projects::start(&projects::try_to_guess());
            } else {
                projects::start(args[2].trim());
            }
        },
        _ => print_help(),
    }
}

fn print_help() {
    println!("Usage: easily [start|stop|create|remove] <project>");
    println!("- start <?project>\t Starts the given project or the project in the current directory");
    println!("- stop <?project>\t Stops the given project or the project in the current directory");
    println!("- restart <?project>\t Restart the given project or the project in the current directory");
    println!("- create Creates a new project");
    println!("- remove <project>\t Removes the project containers from docker");
    println!("- help\t\t\t Shows this help message\n");
}

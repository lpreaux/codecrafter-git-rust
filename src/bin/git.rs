use clap::{Parser, Subcommand};
use anyhow::Result;
use codecrafters_git::repository;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Initialize a new git repository in working directory
    Init,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let command = extract_command(args.command)?;

    match command {
        Commands::Init => {
            repository::init_repository()
        }
    }
    // Uncomment this block to pass the first stage
    // let args: Vec<String> = env::args().collect();
    // if args[1] == "init" {
    //     fs::create_dir(".git").unwrap();
    //     fs::create_dir(".git/objects").unwrap();
    //     fs::create_dir(".git/refs").unwrap();
    //     fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    //     println!("Initialized git directory")
    // } else {
    //     println!("unknown command: {}", args[1])
    // }
}

fn extract_command(opt_commands: Option<Commands>) -> Result<Commands> {
    if let Some(command) = opt_commands {
        return Ok(command);
    }
    Err(anyhow::anyhow!("unknown command: {:?}", opt_commands))
}



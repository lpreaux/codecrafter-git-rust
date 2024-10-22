use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;
use codecrafters_git::repository;
use codecrafters_git::objects;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new git repository in the working directory.
    Init,

    /// Display contents of a repository object.
    CatFile {
        /// Pretty-print the contents of the object
        #[arg(short, default_value_t = false)]
        pretty_print: bool,
        /// The name of the object to show
        object: String,
    },

    /// Compute and optionally write the hash of a file.
    HashObject {
        #[arg(short)]
        write_mode: bool,
        /// Path to the file to hash
        file_path: PathBuf,
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if let Some(command) = args.command {
        match command {
            Commands::Init => repository::init_repository(),
            Commands::CatFile { pretty_print, object } => {
                cat_file(&object, pretty_print)
            },
            Commands::HashObject { write_mode, file_path } => {
                hash_object(&file_path, write_mode)
            },
        }
    } else {
        // clap handles displaying help if no command is provided
        println!("Must supply a command. Try -h (or --help) for help.");
    }
}

fn cat_file(object_name: &str, pretty_print: bool) -> Result<()> {
    let object: objects::Blob = object_name.try_into()?;

    if pretty_print {
        print!("{}", object.content)
    } else {
        print!("{:?}", object)
    }
    Ok(())
}

fn hash_object(path: &PathBuf, write_mode: bool) -> Result<()> {
    let hash = objects::file_to_hash(path, write_mode)?;
    print!("{}", hash);
    Ok(())
}

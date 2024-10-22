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
    /// Initialize a new git repository in working directory.
    Init,

    /// Provide contents or details of repository objects.
    CatFile {
        /// Pretty-print the contents of <object> based on its type
        #[arg(short)]
        pretty_print: bool,
        /// The name of the object to show
        object: String,
    },

    HashObject {
        #[arg(short)]
        write_mode: bool,

        file_path: PathBuf
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Init) => {
            repository::init_repository()
        },
        Some(Commands::CatFile {pretty_print, object}) => {
            cat_file(object, pretty_print)
        },
        Some(Commands::HashObject {write_mode, file_path, }) => {
            hash_object(file_path, write_mode)
        }
        None => panic!("Must supply a command. Try -h (or --help) for help."),
    }
}

fn cat_file(object_name: &String, pretty_print: &bool) -> Result<()> {
    let object: objects::Blob = object_name.try_into()?;

    if *pretty_print {
        print!("{}", object.content)
    } else {
        print!("{:?}", object)
    }
    Ok(())
}

fn hash_object(path: &PathBuf, write_mode: &bool) -> Result<()> {
    let hash = objects::file_to_hash(path, write_mode)?;
    print!("{}", hash);
    Ok(())
}

use std::env;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;
use codecrafters_git::objects::object_base::GitObject;
use codecrafters_git::repository;
use codecrafters_git::objects::object_manager;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
    },

    LsTree {
        #[arg(long)]
        name_only: bool,
        object: String,
    },

    WriteTree {
        #[arg(default_value_os_t = get_working_dir_path())]
        path: PathBuf,
    },

    CommitTree {
        object: String,
        #[arg(short, long)]
        parent: Option<String>,
        #[arg(short, long)]
        message: String,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Init => repository::init_repository(),
        Commands::CatFile { pretty_print, object } => {
            cat_file(&object, pretty_print)
        }
        Commands::HashObject { write_mode, file_path } => {
            hash_object(&file_path, write_mode)
        }
        Commands::LsTree { name_only, object } => {
            ls_tree(&object, name_only)
        }
        Commands::WriteTree { path } => {
            write_tree(&path)
        },
        Commands::CommitTree { object, parent, message } => {
            commit_tree(&object, &parent, &message)
        }
    }
}

fn commit_tree(object_name: &String, parent_name: &Option<String>, message: &String) -> Result<()> {
    let object = object_manager::create_commit(object_name, parent_name, message)?;
    println!("{}", object.get_hash());
    Ok(())
}

fn ls_tree(object_name: &String, name_only: bool) -> Result<()> {
    let object = object_manager::read_object(object_name)?;
    println!("{}", object.get_data()?);
    Ok(())
}

fn write_tree(path: &PathBuf) -> Result<()> {
    let object = object_manager::create_object(path)?;
    println!("{}", object.get_hash());
    Ok(())
}

fn cat_file(object_name: &str, pretty_print: bool) -> Result<()> {
    let object = object_manager::read_object(object_name)?;

    if pretty_print {
        print!("{}", object.get_data()?)
    } else {
        // print!("{:?}", object)
    }
    Ok(())
}

fn hash_object(path: &PathBuf, write_mode: bool) -> Result<()> {
    let object = object_manager::create_object(path)?;
    print!("{}", object.get_hash());
    Ok(())
}

fn get_working_dir_path() -> PathBuf {
    env::current_dir().unwrap()
}
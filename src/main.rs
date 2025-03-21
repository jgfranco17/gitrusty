use clap::{Parser, Subcommand}; // CLI parsing
use git2::{Repository, Error, Status};  // Git operations
use colored::*;

#[derive(Parser)]
#[command(name = "RustGit")]
#[command(version = "0.1")]
#[command(about = "A TUI-based Git client in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Git repository
    Init { path: Option<String> },

    /// Show repository status
    Status,

    /// List commit history
    Log,

    /// Show branches
    Branch,
}

fn main() {
    let cli = Cli::parse(); // Parse CLI args

    match &cli.command {
        Commands::Init { path } => {
            let repo_path = path.clone().unwrap_or_else(|| ".".to_string());
            match Repository::init(repo_path.clone()) {
                Ok(_) => println!("Initialized Git repository in {}", repo_path),
                Err(e) => eprintln!("Error initializing repository: {}", e),
            }
        }
        Commands::Status => match print_status() {
            Ok(_) => {},
            Err(e) => eprintln!("Error getting status: {}", e),
        },
        Commands::Log => match print_log() {
            Ok(_) => {},
            Err(e) => eprintln!("Error getting commit log: {}", e),
        },
        Commands::Branch => match list_branches() {
            Ok(_) => {},
            Err(e) => eprintln!("Error getting branches: {}", e),
        },
    }
}

fn print_status() -> Result<(), Error> {
    let repo = Repository::open(".")?;
    let statuses = repo.statuses(None)?;

    if statuses.is_empty() {
        println!("No changes detected.");
    } else {
        for entry in statuses.iter() {
            let _path = entry.path().unwrap_or("unknown");
            let status_str = get_color_from_status(entry.status());
            if let Some(path) = entry.path() {
                println!("{} {}", status_str, path);
            }
        }
    }
    Ok(())
}

fn get_color_from_status(status: Status) -> ColoredString {
    let status_str = match status {
        s if s.contains(Status::WT_NEW) => "Untracked".cyan(),
        s if s.contains(Status::WT_MODIFIED) => "Modified".yellow(),
        s if s.contains(Status::WT_DELETED) => "Deleted".red(),
        s if s.contains(Status::WT_RENAMED) => "Renamed".magenta(),
        s if s.contains(Status::WT_TYPECHANGE) => "Type Changed".cyan(),
        s if s.contains(Status::IGNORED) => "Ignored".dimmed(),
        s if s.contains(Status::INDEX_NEW) => "Staged (New)".green(),
        s if s.contains(Status::INDEX_MODIFIED) => "Staged (Modified)".green(),
        s if s.contains(Status::INDEX_DELETED) => "Staged (Deleted)".green(),
        s if s.contains(Status::INDEX_RENAMED) => "Staged (Renamed)".green(),
        s if s.contains(Status::INDEX_TYPECHANGE) => "Staged (Type Changed)".green(),
        _ => "Unknown".white(),
    };
    return status_str;
}

fn print_log() -> Result<(), Error> {
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk.take(10) {
        let commit = repo.find_commit(oid?)?;
        println!(
            "{} - {}",
            commit.id(),
            commit.summary().unwrap_or("<No message>")
        );
    }
    Ok(())
}

fn list_branches() -> Result<(), Error> {
    let repo = Repository::open(".")?;
    let branches = repo.branches(None)?;

    for branch in branches {
        let (branch, _) = branch?;
        let name = branch.name()?.unwrap_or("<Unnamed>");
        println!("{}", name);
    }
    Ok(())
}

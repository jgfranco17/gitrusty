use git2::Repository;
use tempfile::tempdir;
use std::fs;

/// Ensure `git init` initializes a repository correctly
#[test]
fn test_git_init() {
    let temp = tempdir().unwrap();
    let repo_path = temp.path().join("repo");
    fs::create_dir(&repo_path).unwrap();

    // Initialize the repository
    let _repo = Repository::init(&repo_path).expect("Failed to initialize repo");

    // Check if `.git` directory was created
    assert!(repo_path.join(".git").exists(), "Repository was not initialized");
}

/// Ensure `git status` detects new untracked files
#[test]
fn test_git_status_untracked_files() {
    let temp = tempdir().unwrap();
    let repo_path = temp.path();
    let repo = Repository::init(&repo_path).expect("Failed to initialize repo");

    // Create an untracked file
    let file_path = repo_path.join("untracked_file.txt");
    fs::write(&file_path, "Test content").unwrap();

    // Get status
    let mut status_opts = git2::StatusOptions::new();
    status_opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut status_opts)).unwrap();

    // Ensure at least one file is untracked
    let untracked = statuses.iter().any(|entry| {
        entry.status().contains(git2::Status::WT_NEW) && entry.path() == Some("untracked_file.txt")
    });

    assert!(untracked, "Untracked file was not detected in git status");
}

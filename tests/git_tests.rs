use git2::Repository;
use tempfile::tempdir;
use std::fs;
use std::path::Path;

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

/// Ensure `git status` detects modified files
#[test]
fn test_git_status_modified_files() {
    let temp = tempdir().unwrap();
    let repo_path = temp.path();
    let repo = Repository::init(&repo_path).expect("Failed to initialize repo");

    // Create and commit a file
    let file_path = repo_path.join("tracked_file.txt");
    fs::write(&file_path, "Initial content").unwrap();
    let mut index = repo.index().unwrap();
    index.add_path(Path::new("tracked_file.txt")).unwrap();
    index.write().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let sig = repo.signature().unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[]).unwrap();

    // Modify the file
    fs::write(&file_path, "Modified content").unwrap();

    // Get status
    let mut status_opts = git2::StatusOptions::new();
    status_opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut status_opts)).unwrap();

    // Ensure file is detected as modified
    let modified = statuses.iter().any(|entry| {
        entry.status().contains(git2::Status::WT_MODIFIED) && entry.path() == Some("tracked_file.txt")
    });

    assert!(modified, "Modified file was not detected in git status");
}

//! Git integration for VS Code-like UI
//!
//! Provides Git status information similar to VS Code's Git integration.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::HashMap;

/// Git file status types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitStatus {
    /// File is unmodified
    Unmodified,
    /// File has been modified but not staged
    Modified,
    /// File has been added to the index
    Added,
    /// File has been deleted but not staged
    Deleted,
    /// File has been renamed
    Renamed,
    /// File has been copied
    Copied,
    /// File has merge conflicts
    Unmerged,
    /// File is untracked
    Untracked,
    /// File status unknown
    Unknown,
}

/// Git integration manager
#[derive(Debug, Clone)]
pub struct GitManager {
    /// Current repository root path
    repo_root: Option<PathBuf>,
    /// Current branch name
    branch: Option<String>,
    /// File statuses
    file_statuses: HashMap<PathBuf, GitStatus>,
    /// Remote tracking branch
    remote_tracking: Option<String>,
    /// Whether the repository has changes
    has_changes: bool,
    /// Number of staged changes
    staged_count: usize,
    /// Number of unstaged changes
    unstaged_count: usize,
    /// Number of untracked files
    untracked_count: usize,
}

impl GitManager {
    /// Create a new Git manager
    pub fn new() -> Self {
        Self {
            repo_root: None,
            branch: None,
            file_statuses: HashMap::new(),
            remote_tracking: None,
            has_changes: false,
            staged_count: 0,
            unstaged_count: 0,
            untracked_count: 0,
        }
    }
    
    /// Detect if the given path is within a Git repository
    pub fn detect_repo(&mut self, path: &Path) {
        // Start with the given path
        let mut current_path = PathBuf::from(path);
        
        // Walk up the directory tree to find .git
        while current_path.parent().is_some() {
            let git_dir = current_path.join(".git");
            if git_dir.exists() && git_dir.is_dir() {
                self.repo_root = Some(current_path.clone());
                self.update_branch();
                self.update_file_statuses();
                return;
            }
            // Move up one directory
            if !current_path.pop() {
                break;
            }
        }
        
        // No Git repository found
        self.repo_root = None;
        self.branch = None;
        self.file_statuses.clear();
    }
    
    /// Update the current branch information
    pub fn update_branch(&mut self) {
        if let Some(repo_path) = &self.repo_root {
            // Run 'git branch --show-current'
            let output = Command::new("git")
                .current_dir(repo_path)
                .args(["branch", "--show-current"])
                .output();
                
            if let Ok(output) = output {
                if output.status.success() {
                    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    self.branch = if branch.is_empty() { None } else { Some(branch) };
                } else {
                    self.branch = None;
                }
            } else {
                self.branch = None;
            }
            
            // Get remote tracking branch
            let tracking_output = Command::new("git")
                .current_dir(repo_path)
                .args(["rev-parse", "--abbrev-ref", "@{upstream}"])
                .output();
                
            if let Ok(output) = tracking_output {
                if output.status.success() {
                    let tracking = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    self.remote_tracking = if tracking.is_empty() { None } else { Some(tracking) };
                } else {
                    self.remote_tracking = None;
                }
            } else {
                self.remote_tracking = None;
            }
        }
    }
    
    /// Update file statuses in the repository
    pub fn update_file_statuses(&mut self) {
        if let Some(repo_path) = &self.repo_root {
            self.file_statuses.clear();
            
            // Run 'git status --porcelain'
            let output = Command::new("git")
                .current_dir(repo_path)
                .args(["status", "--porcelain"])
                .output();
                
            if let Ok(output) = output {
                if output.status.success() {
                    let status_output = String::from_utf8_lossy(&output.stdout);
                    let mut staged_count = 0;
                    let mut unstaged_count = 0;
                    let mut untracked_count = 0;
                    
                    for line in status_output.lines() {
                        if line.len() < 3 {
                            continue;
                        }
                        
                        let status_code = &line[0..2];
                        let file_path = line[3..].trim();
                        let full_path = repo_path.join(file_path);
                        
                        // Parse status code according to git status --porcelain format
                        let status = match status_code {
                            "M " => { staged_count += 1; GitStatus::Added },
                            " M" => { unstaged_count += 1; GitStatus::Modified },
                            "MM" => { staged_count += 1; unstaged_count += 1; GitStatus::Modified },
                            "A " => { staged_count += 1; GitStatus::Added },
                            " A" => { unstaged_count += 1; GitStatus::Added },
                            "D " => { staged_count += 1; GitStatus::Deleted },
                            " D" => { unstaged_count += 1; GitStatus::Deleted },
                            "R " => { staged_count += 1; GitStatus::Renamed },
                            "C " => { staged_count += 1; GitStatus::Copied },
                            "??" => { untracked_count += 1; GitStatus::Untracked },
                            "UU" => GitStatus::Unmerged,
                            _ => GitStatus::Unknown,
                        };
                        
                        self.file_statuses.insert(full_path, status);
                    }
                    
                    self.has_changes = staged_count > 0 || unstaged_count > 0 || untracked_count > 0;
                    self.staged_count = staged_count;
                    self.unstaged_count = unstaged_count;
                    self.untracked_count = untracked_count;
                }
            }
        }
    }
    
    /// Get the current branch name
    pub fn branch(&self) -> Option<&str> {
        self.branch.as_deref()
    }
    
    /// Get the status of a file
    pub fn file_status(&self, path: &Path) -> GitStatus {
        self.file_statuses.get(path).cloned().unwrap_or(GitStatus::Unmodified)
    }
    
    /// Check if a repository has been detected
    pub fn has_repo(&self) -> bool {
        self.repo_root.is_some()
    }
    
    /// Check if the repository has changes
    pub fn has_changes(&self) -> bool {
        self.has_changes
    }
    
    /// Get the number of staged changes
    pub fn staged_count(&self) -> usize {
        self.staged_count
    }
    
    /// Get the number of unstaged changes
    pub fn unstaged_count(&self) -> usize {
        self.unstaged_count
    }
    
    /// Get the number of untracked files
    pub fn untracked_count(&self) -> usize {
        self.untracked_count
    }
    
    /// Get total number of changes
    pub fn total_changes(&self) -> usize {
        self.staged_count + self.unstaged_count + self.untracked_count
    }
    
    /// Get status summary formatted like VS Code (e.g., "+10 ~5 -2")
    pub fn status_summary(&self) -> String {
        if !self.has_changes {
            return String::new();
        }
        
        let mut parts = Vec::new();
        
        let added = self.file_statuses.values().filter(|s| matches!(s, GitStatus::Added)).count();
        let modified = self.file_statuses.values().filter(|s| matches!(s, GitStatus::Modified)).count();
        let deleted = self.file_statuses.values().filter(|s| matches!(s, GitStatus::Deleted)).count();
        
        if added > 0 {
            parts.push(format!("+{}", added));
        }
        if modified > 0 {
            parts.push(format!("~{}", modified));
        }
        if deleted > 0 {
            parts.push(format!("-{}", deleted));
        }
        
        parts.join(" ")
    }
}
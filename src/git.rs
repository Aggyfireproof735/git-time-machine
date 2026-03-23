use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct GitEntry {
    pub hash: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub relative_time: String,
}

pub struct GitManager {
    repo_path: String,
}

impl GitManager {
    pub fn new() -> Result<Self> {
        // Check if we're in a git repository
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()
            .context("Failed to execute git command. Are you in a git repository?")?;

        if !output.status.success() {
            anyhow::bail!("Not a git repository");
        }

        let repo_path = String::from_utf8(output.stdout)?
            .trim()
            .to_string();

        Ok(Self { repo_path })
    }

    pub fn get_reflog_entries(&self, show_all: bool) -> Result<Vec<GitEntry>> {
        let limit = if show_all { "1000" } else { "50" };
        
        let output = Command::new("git")
            .args([
                "reflog",
                "--format=%H|%s|%ct",
                &format!("-n{}", limit),
            ])
            .output()
            .context("Failed to get git reflog")?;

        if !output.status.success() {
            anyhow::bail!("Failed to read git reflog");
        }

        let reflog_output = String::from_utf8(output.stdout)?;
        let mut entries = Vec::new();

        for line in reflog_output.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 3 {
                let hash = parts[0].to_string();
                let message = parts[1].to_string();
                let timestamp_str = parts[2];

                if let Ok(timestamp_secs) = timestamp_str.parse::<i64>() {
                    let timestamp = DateTime::from_timestamp(timestamp_secs, 0)
                        .unwrap_or_else(|| Utc::now());
                    let relative_time = Self::format_relative_time(&timestamp);

                    entries.push(GitEntry {
                        hash,
                        message,
                        timestamp,
                        relative_time,
                    });
                }
            }
        }

        Ok(entries)
    }

    pub fn restore_to_commit(&self, commit_hash: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["reset", "--hard", commit_hash])
            .output()
            .context("Failed to restore to commit")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to restore: {}", error);
        }

        Ok(())
    }

    fn format_relative_time(timestamp: &DateTime<Utc>) -> String {
        let now = Utc::now();
        let duration = now.signed_duration_since(*timestamp);

        let seconds = duration.num_seconds();
        let minutes = duration.num_minutes();
        let hours = duration.num_hours();
        let days = duration.num_days();

        if seconds < 60 {
            format!("{}s ago", seconds)
        } else if minutes < 60 {
            format!("{}m ago", minutes)
        } else if hours < 24 {
            format!("{}h ago", hours)
        } else if days < 7 {
            format!("{}d ago", days)
        } else if days < 30 {
            format!("{}w ago", days / 7)
        } else if days < 365 {
            format!("{}mo ago", days / 30)
        } else {
            format!("{}y ago", days / 365)
        }
    }
}

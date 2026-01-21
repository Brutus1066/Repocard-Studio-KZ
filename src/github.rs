// RepoCard Studio - GitHub API Module
// LAZYFROG (of KZ) — kindware.dev

use crate::{CommitInfo, RepoMetadata, LicenseInfo, OwnerInfo};
use reqwest::Client;
use serde::Deserialize;

const USER_AGENT: &str = "RepoCard-Studio/1.0 (kindware.dev)";

/// Parse owner and repo from various GitHub URL formats
fn parse_repo_url(url: &str) -> Result<(String, String), String> {
    let url = url.trim();
    
    // Handle full URLs like https://github.com/owner/repo
    if url.starts_with("https://github.com/") || url.starts_with("http://github.com/") {
        let path = url
            .replace("https://github.com/", "")
            .replace("http://github.com/", "");
        let parts: Vec<&str> = path.trim_matches('/').split('/').collect();
        if parts.len() >= 2 {
            return Ok((parts[0].to_string(), parts[1].to_string()));
        }
    }
    
    // Handle short format like owner/repo
    if url.contains('/') && !url.contains("://") {
        let parts: Vec<&str> = url.split('/').collect();
        if parts.len() == 2 {
            return Ok((parts[0].to_string(), parts[1].to_string()));
        }
    }
    
    Err(format!("Invalid GitHub URL or repo format: {}", url))
}

#[derive(Debug, Deserialize)]
struct GitHubRepoResponse {
    name: String,
    full_name: String,
    description: Option<String>,
    html_url: String,
    stargazers_count: u32,
    forks_count: u32,
    watchers_count: u32,
    open_issues_count: u32,
    language: Option<String>,
    topics: Option<Vec<String>>,
    created_at: String,
    updated_at: String,
    pushed_at: String,
    default_branch: String,
    license: Option<GitHubLicense>,
    owner: GitHubOwner,
}

#[derive(Debug, Deserialize)]
struct GitHubLicense {
    key: String,
    name: String,
    spdx_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubOwner {
    login: String,
    avatar_url: String,
    html_url: String,
}

#[derive(Debug, Deserialize)]
struct GitHubCommit {
    sha: String,
    commit: GitHubCommitDetails,
}

#[derive(Debug, Deserialize)]
struct GitHubCommitDetails {
    message: String,
    author: GitHubCommitAuthor,
}

#[derive(Debug, Deserialize)]
struct GitHubCommitAuthor {
    name: String,
    email: String,
    date: String,
}

/// Fetch repository metadata from GitHub API
pub async fn fetch_repository_metadata(repo_url: &str) -> Result<RepoMetadata, String> {
    let (owner, repo) = parse_repo_url(repo_url)?;
    let api_url = format!("https://api.github.com/repos/{}/{}", owner, repo);
    
    let client = Client::new();
    let response = client
        .get(&api_url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("GitHub API error ({}): {}", status, body));
    }
    
    let github_repo: GitHubRepoResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse GitHub response: {}", e))?;
    
    Ok(RepoMetadata {
        name: github_repo.name,
        full_name: github_repo.full_name,
        description: github_repo.description,
        html_url: github_repo.html_url,
        stargazers_count: github_repo.stargazers_count,
        forks_count: github_repo.forks_count,
        watchers_count: github_repo.watchers_count,
        open_issues_count: github_repo.open_issues_count,
        language: github_repo.language,
        topics: github_repo.topics.unwrap_or_default(),
        created_at: github_repo.created_at,
        updated_at: github_repo.updated_at,
        pushed_at: github_repo.pushed_at,
        default_branch: github_repo.default_branch,
        license: github_repo.license.map(|l| LicenseInfo {
            key: l.key,
            name: l.name,
            spdx_id: l.spdx_id,
        }),
        owner: OwnerInfo {
            login: github_repo.owner.login,
            avatar_url: github_repo.owner.avatar_url,
            html_url: github_repo.owner.html_url,
        },
    })
}

/// Fetch recent commits from GitHub repository
pub async fn fetch_recent_commits(repo_url: &str, count: u32) -> Result<Vec<CommitInfo>, String> {
    let (owner, repo) = parse_repo_url(repo_url)?;
    let api_url = format!(
        "https://api.github.com/repos/{}/{}/commits?per_page={}",
        owner, repo, count.min(100)
    );
    
    let client = Client::new();
    let response = client
        .get(&api_url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("GitHub API error ({}): {}", status, body));
    }
    
    let commits: Vec<GitHubCommit> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse commits: {}", e))?;
    
    Ok(commits
        .into_iter()
        .map(|c| CommitInfo {
            sha: c.sha[..7].to_string(),
            message: c.commit.message.lines().next().unwrap_or("").to_string(),
            author_name: c.commit.author.name,
            author_email: c.commit.author.email,
            date: c.commit.author.date,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_repo_url_full() {
        let result = parse_repo_url("https://github.com/microsoft/vscode");
        assert!(result.is_ok());
        let (owner, repo) = result.unwrap();
        assert_eq!(owner, "microsoft");
        assert_eq!(repo, "vscode");
    }

    #[test]
    fn test_parse_repo_url_short() {
        let result = parse_repo_url("facebook/react");
        assert!(result.is_ok());
        let (owner, repo) = result.unwrap();
        assert_eq!(owner, "facebook");
        assert_eq!(repo, "react");
    }

    #[test]
    fn test_parse_repo_url_invalid() {
        let result = parse_repo_url("invalid-url");
        assert!(result.is_err());
    }
}

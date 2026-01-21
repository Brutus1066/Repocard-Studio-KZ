// RepoCard Studio - Rust Backend
// LAZYFROG (creator of KZ) — kindware.dev

use serde::{Deserialize, Serialize};

mod github;
mod export;
mod templates;

pub use github::*;
pub use export::*;
pub use templates::*;

/// Repository metadata from GitHub API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoMetadata {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub watchers_count: u32,
    pub open_issues_count: u32,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub default_branch: String,
    pub license: Option<LicenseInfo>,
    pub owner: OwnerInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub key: String,
    pub name: String,
    pub spdx_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerInfo {
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub sha: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    pub output_dir: String,
    pub include_attribution: bool,
    pub template_id: String,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub success: bool,
    pub output_path: String,
    pub files: Vec<String>,
    pub error: Option<String>,
}

/// Fetch repository metadata from GitHub
#[tauri::command]
async fn fetch_repo(repo_url: String) -> Result<RepoMetadata, String> {
    github::fetch_repository_metadata(&repo_url).await
}

/// Fetch recent commits from a repository
#[tauri::command]
async fn fetch_commits(repo_url: String, count: Option<u32>) -> Result<Vec<CommitInfo>, String> {
    github::fetch_recent_commits(&repo_url, count.unwrap_or(20)).await
}

/// Generate SVG card from template
#[tauri::command]
fn generate_svg_card(
    metadata: RepoMetadata, 
    template_id: String, 
    include_attribution: bool,
    primary_color: Option<String>,
    secondary_color: Option<String>,
) -> Result<String, String> {
    templates::generate_svg(&metadata, &template_id, include_attribution, primary_color, secondary_color)
}

/// Convert SVG to PNG using resvg
#[tauri::command]
fn svg_to_png(svg_content: String, width: Option<u32>) -> Result<Vec<u8>, String> {
    templates::rasterize_svg(&svg_content, width.unwrap_or(1200))
}

/// Generate README snippet
#[tauri::command]
fn create_readme_snippet(metadata: RepoMetadata, include_attribution: bool) -> String {
    templates::generate_readme_snippet(&metadata, include_attribution)
}

/// Generate release notes draft from commits
#[tauri::command]
fn generate_release_notes(
    metadata: RepoMetadata, 
    commits: Vec<CommitInfo>, 
    version: Option<String>,
    include_attribution: bool,
) -> String {
    templates::generate_release_notes_draft(&metadata, &commits, version, include_attribution)
}

/// Generate press kit overview
#[tauri::command]
fn generate_press_kit_overview(metadata: RepoMetadata, include_attribution: bool) -> String {
    templates::generate_press_kit(&metadata, include_attribution)
}

/// Export full share kit to filesystem
#[tauri::command]
async fn export_share_kit(
    metadata: RepoMetadata,
    commits: Vec<CommitInfo>,
    options: ExportOptions,
) -> Result<ExportResult, String> {
    export::export_full_share_kit(&metadata, &commits, &options).await
}

/// Get default export directory
#[tauri::command]
fn get_default_export_dir() -> Result<String, String> {
    dirs::download_dir()
        .or_else(dirs::document_dir)
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "Could not determine default export directory".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            fetch_repo,
            fetch_commits,
            generate_svg_card,
            svg_to_png,
            create_readme_snippet,
            generate_release_notes,
            generate_press_kit_overview,
            export_share_kit,
            get_default_export_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running RepoCard Studio");
}

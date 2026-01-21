// RepoCard Studio - Export Module
// LAZYFROG (creator of KZ) — kindware.dev

use crate::{CommitInfo, ExportOptions, ExportResult, RepoMetadata};
use crate::templates;
use std::fs;
use std::path::PathBuf;

/// Export full share kit to filesystem
pub async fn export_full_share_kit(
    metadata: &RepoMetadata,
    commits: &[CommitInfo],
    options: &ExportOptions,
) -> Result<ExportResult, String> {
    let base_path = PathBuf::from(&options.output_dir);
    let share_kit_path = base_path.join("share-kit");
    let press_kit_path = share_kit_path.join("press-kit");
    let screenshots_path = press_kit_path.join("screenshots");
    
    // Create directory structure
    fs::create_dir_all(&screenshots_path)
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    
    let mut files: Vec<String> = Vec::new();
    
    // Generate and save SVG card
    let svg_content = templates::generate_svg(
        metadata,
        &options.template_id,
        options.include_attribution,
        options.primary_color.clone(),
        options.secondary_color.clone(),
    )?;
    
    let svg_path = share_kit_path.join("repo-card.svg");
    fs::write(&svg_path, &svg_content)
        .map_err(|e| format!("Failed to write SVG: {}", e))?;
    files.push("repo-card.svg".to_string());
    
    // Generate and save PNG card
    let png_data = templates::rasterize_svg(&svg_content, 1200)?;
    let png_path = share_kit_path.join("repo-card.png");
    fs::write(&png_path, &png_data)
        .map_err(|e| format!("Failed to write PNG: {}", e))?;
    files.push("repo-card.png".to_string());
    
    // Generate and save README snippet
    let readme_snippet = templates::generate_readme_snippet(metadata, options.include_attribution);
    let readme_path = share_kit_path.join("README-snippet.md");
    fs::write(&readme_path, &readme_snippet)
        .map_err(|e| format!("Failed to write README snippet: {}", e))?;
    files.push("README-snippet.md".to_string());
    
    // Generate and save release notes
    let release_notes = templates::generate_release_notes_draft(
        metadata,
        commits,
        None,
        options.include_attribution,
    );
    let notes_path = share_kit_path.join("release-notes-draft.md");
    fs::write(&notes_path, &release_notes)
        .map_err(|e| format!("Failed to write release notes: {}", e))?;
    files.push("release-notes-draft.md".to_string());
    
    // Generate and save press kit overview
    let press_kit = templates::generate_press_kit(metadata, options.include_attribution);
    let overview_path = press_kit_path.join("overview.md");
    fs::write(&overview_path, &press_kit)
        .map_err(|e| format!("Failed to write press kit: {}", e))?;
    files.push("press-kit/overview.md".to_string());
    
    // Create .gitkeep in screenshots folder
    let gitkeep_path = screenshots_path.join(".gitkeep");
    fs::write(&gitkeep_path, "")
        .map_err(|e| format!("Failed to write .gitkeep: {}", e))?;
    files.push("press-kit/screenshots/.gitkeep".to_string());
    
    Ok(ExportResult {
        success: true,
        output_path: share_kit_path.to_string_lossy().to_string(),
        files,
        error: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LicenseInfo, OwnerInfo};
    use tempfile::TempDir;

    fn sample_metadata() -> RepoMetadata {
        RepoMetadata {
            name: "test-repo".to_string(),
            full_name: "owner/test-repo".to_string(),
            description: Some("A test repository".to_string()),
            html_url: "https://github.com/owner/test-repo".to_string(),
            stargazers_count: 100,
            forks_count: 10,
            watchers_count: 50,
            open_issues_count: 5,
            language: Some("Rust".to_string()),
            topics: vec!["test".to_string()],
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-06-01T00:00:00Z".to_string(),
            pushed_at: "2024-06-01T00:00:00Z".to_string(),
            default_branch: "main".to_string(),
            license: Some(LicenseInfo {
                key: "mit".to_string(),
                name: "MIT License".to_string(),
                spdx_id: Some("MIT".to_string()),
            }),
            owner: OwnerInfo {
                login: "owner".to_string(),
                avatar_url: "https://github.com/owner.png".to_string(),
                html_url: "https://github.com/owner".to_string(),
            },
        }
    }

    #[tokio::test]
    async fn test_export_creates_files() {
        let temp_dir = TempDir::new().unwrap();
        let metadata = sample_metadata();
        let commits = vec![
            CommitInfo {
                sha: "abc1234".to_string(),
                message: "feat: initial commit".to_string(),
                author_name: "Test".to_string(),
                author_email: "test@example.com".to_string(),
                date: "2024-01-01T00:00:00Z".to_string(),
            },
        ];
        
        let options = ExportOptions {
            output_dir: temp_dir.path().to_string_lossy().to_string(),
            include_attribution: true,
            template_id: "modern".to_string(),
            primary_color: None,
            secondary_color: None,
        };
        
        let result = export_full_share_kit(&metadata, &commits, &options).await;
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(result.success);
        assert!(!result.files.is_empty());
        
        // Verify files exist
        let share_kit = temp_dir.path().join("share-kit");
        assert!(share_kit.join("repo-card.svg").exists());
        assert!(share_kit.join("repo-card.png").exists());
        assert!(share_kit.join("README-snippet.md").exists());
        assert!(share_kit.join("release-notes-draft.md").exists());
        assert!(share_kit.join("press-kit/overview.md").exists());
    }
}

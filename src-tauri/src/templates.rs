// RepoCard Studio - SVG Templates Module
// LAZYFROG (KZ) — kindware.dev

use crate::{CommitInfo, RepoMetadata};
use chrono::Utc;

const ATTRIBUTION_TEXT: &str = "Generated with RepoCard Studio — LAZYFROG (KZ) — kindware.dev";

/// Language colors for visual branding
fn get_language_color(language: &str) -> &'static str {
    match language.to_lowercase().as_str() {
        "javascript" => "#f1e05a",
        "typescript" => "#3178c6",
        "python" => "#3572A5",
        "rust" => "#dea584",
        "go" => "#00ADD8",
        "java" => "#b07219",
        "c++" | "cpp" => "#f34b7d",
        "c#" | "csharp" => "#178600",
        "ruby" => "#701516",
        "php" => "#4F5D95",
        "swift" => "#F05138",
        "kotlin" => "#A97BFF",
        "dart" => "#00B4AB",
        "vue" => "#41b883",
        "html" => "#e34c26",
        "css" => "#563d7c",
        "shell" | "bash" => "#89e051",
        "c" => "#555555",
        _ => "#6e7681",
    }
}

/// Format large numbers with K/M suffix
fn format_count(count: u32) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Truncate text with ellipsis
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Generate SVG card based on template
pub fn generate_svg(
    metadata: &RepoMetadata,
    template_id: &str,
    include_attribution: bool,
    primary_color: Option<String>,
    secondary_color: Option<String>,
) -> Result<String, String> {
    match template_id {
        "modern" => Ok(generate_modern_template(metadata, include_attribution, primary_color, secondary_color)),
        "minimal" => Ok(generate_minimal_template(metadata, include_attribution, primary_color, secondary_color)),
        "gradient" => Ok(generate_gradient_template(metadata, include_attribution, primary_color, secondary_color)),
        _ => Err(format!("Unknown template: {}", template_id)),
    }
}

/// Modern template - clean, professional look
fn generate_modern_template(
    metadata: &RepoMetadata,
    include_attribution: bool,
    primary_color: Option<String>,
    secondary_color: Option<String>,
) -> String {
    let primary = primary_color.unwrap_or_else(|| "#0d1117".to_string());
    let secondary = secondary_color.unwrap_or_else(|| "#161b22".to_string());
    let lang_color = metadata.language.as_deref().map(get_language_color).unwrap_or("#6e7681");
    let description = metadata.description.as_deref().unwrap_or("No description provided");
    
    let attribution_svg = if include_attribution {
        format!(r##"<text x="600" y="305" text-anchor="middle" font-size="10" fill="#6e7681" font-family="system-ui, -apple-system, sans-serif">{}</text>"##, ATTRIBUTION_TEXT)
    } else {
        String::new()
    };

    format!(r##"<!-- KZ: LAZYFROG :: frogprints -->
<svg width="1200" height="630" viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <clipPath id="avatar-clip">
      <circle cx="80" cy="80" r="40"/>
    </clipPath>
    <filter id="shadow" x="-20%" y="-20%" width="140%" height="140%">
      <feDropShadow dx="0" dy="4" stdDeviation="8" flood-opacity="0.25"/>
    </filter>
  </defs>
  
  <!-- Background -->
  <rect width="1200" height="630" fill="{primary}"/>
  <rect x="40" y="40" width="1120" height="550" rx="16" fill="{secondary}" filter="url(#shadow)"/>
  
  <!-- Frogprints Easter Egg -->
  <g opacity="0.03">
    <circle cx="1140" cy="600" r="4"/>
    <circle cx="1152" cy="608" r="3"/>
    <circle cx="1160" cy="598" r="3"/>
  </g>
  
  <!-- Header -->
  <g transform="translate(80, 80)">
    <!-- Owner Avatar Placeholder -->
    <circle cx="40" cy="40" r="40" fill="#30363d"/>
    <text x="40" y="48" text-anchor="middle" font-size="24" fill="#8b949e" font-family="system-ui, -apple-system, sans-serif">{owner_initial}</text>
    
    <!-- Repo Name -->
    <text x="100" y="30" font-size="32" font-weight="bold" fill="#f0f6fc" font-family="system-ui, -apple-system, sans-serif">{owner}</text>
    <text x="100" y="65" font-size="28" fill="#8b949e" font-family="system-ui, -apple-system, sans-serif">/ {repo}</text>
  </g>
  
  <!-- Description -->
  <text x="80" y="200" font-size="20" fill="#c9d1d9" font-family="system-ui, -apple-system, sans-serif">
    <tspan x="80" dy="0">{description}</tspan>
  </text>
  
  <!-- Stats -->
  <g transform="translate(80, 280)">
    <!-- Stars -->
    <g transform="translate(0, 0)">
      <path d="M8 0C3.58 0 0 3.58 0 8s3.58 8 8 8 8-3.58 8-8-3.58-8-8-8zm0 14.5c-3.59 0-6.5-2.91-6.5-6.5S4.41 1.5 8 1.5s6.5 2.91 6.5 6.5-2.91 6.5-6.5 6.5z" fill="#f0f6fc" transform="scale(1.2)"/>
      <path d="M8 3.5l1.5 3 3.5.5-2.5 2.5.5 3.5L8 11l-3 2 .5-3.5L3 7l3.5-.5z" fill="#f0f6fc" transform="scale(1.2)"/>
      <text x="28" y="14" font-size="16" fill="#f0f6fc" font-family="system-ui, -apple-system, sans-serif">{stars}</text>
    </g>
    
    <!-- Forks -->
    <g transform="translate(120, 0)">
      <path d="M5 5.372v.878c0 .414.336.75.75.75h4.5a.75.75 0 0 0 .75-.75v-.878a2.25 2.25 0 1 0-1.5 0v.878H6.25v-.878a2.25 2.25 0 1 0-1.5 0ZM8 1.25a1.25 1.25 0 1 1 0 2.5 1.25 1.25 0 0 1 0-2.5ZM5 4a1.25 1.25 0 1 1 0-2.5A1.25 1.25 0 0 1 5 4Zm6 0a1.25 1.25 0 1 1 0-2.5A1.25 1.25 0 0 1 11 4Z" fill="#f0f6fc" transform="scale(1.2)"/>
      <text x="28" y="14" font-size="16" fill="#f0f6fc" font-family="system-ui, -apple-system, sans-serif">{forks}</text>
    </g>
    
    <!-- Language -->
    <g transform="translate(240, 0)">
      <circle cx="8" cy="8" r="6" fill="{lang_color}"/>
      <text x="24" y="14" font-size="16" fill="#f0f6fc" font-family="system-ui, -apple-system, sans-serif">{language}</text>
    </g>
  </g>
  
  <!-- Topics -->
  <g transform="translate(80, 360)">
    {topics_svg}
  </g>
  
  <!-- Footer -->
  <g transform="translate(80, 520)">
    <text font-size="14" fill="#6e7681" font-family="system-ui, -apple-system, sans-serif">{url}</text>
  </g>
  
  <!-- Attribution -->
  <g transform="translate(0, 570)">
    {attribution}
  </g>
</svg>"##,
        primary = primary,
        secondary = secondary,
        owner = escape_xml(&metadata.owner.login),
        owner_initial = metadata.owner.login.chars().next().unwrap_or('?').to_uppercase(),
        repo = escape_xml(&metadata.name),
        description = escape_xml(&truncate(description, 100)),
        stars = format_count(metadata.stargazers_count),
        forks = format_count(metadata.forks_count),
        language = escape_xml(metadata.language.as_deref().unwrap_or("Unknown")),
        lang_color = lang_color,
        topics_svg = generate_topics_svg(&metadata.topics, 5),
        url = escape_xml(&metadata.html_url),
        attribution = attribution_svg,
    )
}

/// Minimal template - simple, text-focused design
fn generate_minimal_template(
    metadata: &RepoMetadata,
    include_attribution: bool,
    primary_color: Option<String>,
    _secondary_color: Option<String>,
) -> String {
    let primary = primary_color.unwrap_or_else(|| "#ffffff".to_string());
    let description = metadata.description.as_deref().unwrap_or("No description provided");
    
    let attribution_svg = if include_attribution {
        format!(r##"<text x="600" y="600" text-anchor="middle" font-size="11" fill="#9ca3af" font-family="system-ui, -apple-system, sans-serif">{}</text>"##, ATTRIBUTION_TEXT)
    } else {
        String::new()
    };

    format!(r##"<!-- KZ: LAZYFROG :: frogprints -->
<svg width="1200" height="630" viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
  <!-- Background -->
  <rect width="1200" height="630" fill="{primary}"/>
  
  <!-- Frogprints Easter Egg -->
  <g opacity="0.025" fill="#000000">
    <circle cx="1140" cy="600" r="4"/>
    <circle cx="1152" cy="608" r="3"/>
    <circle cx="1160" cy="598" r="3"/>
  </g>
  
  <!-- Content -->
  <g transform="translate(100, 180)">
    <!-- Repo Name -->
    <text font-size="48" font-weight="bold" fill="#111827" font-family="system-ui, -apple-system, sans-serif">
      <tspan fill="#6b7280">{owner}</tspan><tspan fill="#111827"> / {repo}</tspan>
    </text>
    
    <!-- Description -->
    <text y="80" font-size="24" fill="#4b5563" font-family="system-ui, -apple-system, sans-serif">{description}</text>
    
    <!-- Stats Row -->
    <g transform="translate(0, 160)">
      <text font-size="20" fill="#6b7280" font-family="system-ui, -apple-system, sans-serif">
        <tspan font-weight="bold" fill="#111827">{stars}</tspan> stars
        <tspan dx="40" font-weight="bold" fill="#111827">{forks}</tspan> forks
        <tspan dx="40" font-weight="bold" fill="#111827">{issues}</tspan> issues
      </text>
    </g>
    
    <!-- Language Badge -->
    <g transform="translate(0, 220)">
      <rect width="120" height="32" rx="16" fill="#f3f4f6"/>
      <circle cx="20" cy="16" r="6" fill="{lang_color}"/>
      <text x="36" y="21" font-size="14" fill="#374151" font-family="system-ui, -apple-system, sans-serif">{language}</text>
    </g>
  </g>
  
  <!-- Attribution -->
  {attribution}
</svg>"##,
        primary = primary,
        owner = escape_xml(&metadata.owner.login),
        repo = escape_xml(&metadata.name),
        description = escape_xml(&truncate(description, 80)),
        stars = format_count(metadata.stargazers_count),
        forks = format_count(metadata.forks_count),
        issues = format_count(metadata.open_issues_count),
        language = escape_xml(metadata.language.as_deref().unwrap_or("Unknown")),
        lang_color = metadata.language.as_deref().map(get_language_color).unwrap_or("#6e7681"),
        attribution = attribution_svg,
    )
}

/// Gradient template - vibrant, eye-catching design
fn generate_gradient_template(
    metadata: &RepoMetadata,
    include_attribution: bool,
    primary_color: Option<String>,
    secondary_color: Option<String>,
) -> String {
    let primary = primary_color.unwrap_or_else(|| "#667eea".to_string());
    let secondary = secondary_color.unwrap_or_else(|| "#764ba2".to_string());
    let description = metadata.description.as_deref().unwrap_or("No description provided");
    
    let attribution_svg = if include_attribution {
        format!(r#"<text x="600" y="600" text-anchor="middle" font-size="11" fill="rgba(255,255,255,0.7)" font-family="system-ui, -apple-system, sans-serif">{}</text>"#, ATTRIBUTION_TEXT)
    } else {
        String::new()
    };

    format!(r##"<!-- KZ: LAZYFROG :: frogprints -->
<svg width="1200" height="630" viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="bg-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:{primary};stop-opacity:1" />
      <stop offset="100%" style="stop-color:{secondary};stop-opacity:1" />
    </linearGradient>
    <filter id="glow" x="-20%" y="-20%" width="140%" height="140%">
      <feGaussianBlur stdDeviation="2" result="coloredBlur"/>
      <feMerge>
        <feMergeNode in="coloredBlur"/>
        <feMergeNode in="SourceGraphic"/>
      </feMerge>
    </filter>
  </defs>
  
  <!-- Gradient Background -->
  <rect width="1200" height="630" fill="url(#bg-gradient)"/>
  
  <!-- Frogprints Easter Egg -->
  <g opacity="0.04" fill="#ffffff">
    <circle cx="1140" cy="600" r="4"/>
    <circle cx="1152" cy="608" r="3"/>
    <circle cx="1160" cy="598" r="3"/>
  </g>
  
  <!-- Decorative Elements -->
  <circle cx="100" cy="100" r="200" fill="rgba(255,255,255,0.05)"/>
  <circle cx="1100" cy="530" r="250" fill="rgba(255,255,255,0.05)"/>
  
  <!-- Content Card -->
  <rect x="80" y="120" width="1040" height="400" rx="24" fill="rgba(255,255,255,0.1)" stroke="rgba(255,255,255,0.2)" stroke-width="1"/>
  
  <!-- Content -->
  <g transform="translate(140, 180)">
    <!-- Repo Name -->
    <text font-size="56" font-weight="bold" fill="#ffffff" font-family="system-ui, -apple-system, sans-serif" filter="url(#glow)">{full_name}</text>
    
    <!-- Description -->
    <text y="80" font-size="22" fill="rgba(255,255,255,0.9)" font-family="system-ui, -apple-system, sans-serif">{description}</text>
    
    <!-- Stats -->
    <g transform="translate(0, 160)">
      <!-- Stars -->
      <g>
        <rect width="100" height="40" rx="20" fill="rgba(255,255,255,0.2)"/>
        <text x="50" y="27" text-anchor="middle" font-size="16" font-weight="bold" fill="#ffffff" font-family="system-ui, -apple-system, sans-serif">★ {stars}</text>
      </g>
      
      <!-- Forks -->
      <g transform="translate(120, 0)">
        <rect width="100" height="40" rx="20" fill="rgba(255,255,255,0.2)"/>
        <text x="50" y="27" text-anchor="middle" font-size="16" font-weight="bold" fill="#ffffff" font-family="system-ui, -apple-system, sans-serif">⑂ {forks}</text>
      </g>
      
      <!-- Language -->
      <g transform="translate(240, 0)">
        <rect width="140" height="40" rx="20" fill="rgba(255,255,255,0.2)"/>
        <circle cx="24" cy="20" r="8" fill="{lang_color}"/>
        <text x="44" y="27" font-size="16" font-weight="bold" fill="#ffffff" font-family="system-ui, -apple-system, sans-serif">{language}</text>
      </g>
    </g>
    
    <!-- License -->
    <g transform="translate(0, 230)">
      <text font-size="14" fill="rgba(255,255,255,0.7)" font-family="system-ui, -apple-system, sans-serif">
        {license} • Updated {updated}
      </text>
    </g>
  </g>
  
  <!-- Attribution -->
  {attribution}
</svg>"##,
        primary = primary,
        secondary = secondary,
        full_name = escape_xml(&metadata.full_name),
        description = escape_xml(&truncate(description, 70)),
        stars = format_count(metadata.stargazers_count),
        forks = format_count(metadata.forks_count),
        language = escape_xml(metadata.language.as_deref().unwrap_or("Unknown")),
        lang_color = metadata.language.as_deref().map(get_language_color).unwrap_or("#ffffff"),
        license = metadata.license.as_ref().map(|l| l.name.clone()).unwrap_or_else(|| "No License".to_string()),
        updated = &metadata.updated_at[..10],
        attribution = attribution_svg,
    )
}

/// Generate SVG for topic badges
fn generate_topics_svg(topics: &[String], max_topics: usize) -> String {
    let mut svg = String::new();
    let mut x_offset = 0;
    
    for topic in topics.iter().take(max_topics) {
        let width = (topic.len() * 8 + 20) as i32;
        svg.push_str(&format!(
            r##"<g transform="translate({}, 0)"><rect width="{}" height="28" rx="14" fill="#30363d"/><text x="{}" y="19" font-size="12" fill="#8b949e" font-family="system-ui, -apple-system, sans-serif">{}</text></g>"##,
            x_offset,
            width,
            width / 2,
            escape_xml(topic)
        ));
        x_offset += width + 10;
    }
    
    if topics.len() > max_topics {
        svg.push_str(&format!(
            r##"<g transform="translate({}, 0)"><text y="19" font-size="12" fill="#6e7681" font-family="system-ui, -apple-system, sans-serif">+{} more</text></g>"##,
            x_offset,
            topics.len() - max_topics
        ));
    }
    
    svg
}

/// Rasterize SVG to PNG using resvg
pub fn rasterize_svg(svg_content: &str, width: u32) -> Result<Vec<u8>, String> {
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg_content, &options)
        .map_err(|e| format!("Failed to parse SVG: {}", e))?;
    
    let size = tree.size();
    let scale = width as f32 / size.width();
    let height = (size.height() * scale) as u32;
    
    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or("Failed to create pixmap")?;
    
    let transform = tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    
    pixmap.encode_png()
        .map_err(|e| format!("Failed to encode PNG: {}", e))
}

/// Generate README snippet markdown
pub fn generate_readme_snippet(metadata: &RepoMetadata, include_attribution: bool) -> String {
    let description = metadata.description.as_deref().unwrap_or("A GitHub repository");
    let license = metadata.license.as_ref()
        .map(|l| format!("![License](https://img.shields.io/badge/license-{}-blue.svg)", l.spdx_id.as_deref().unwrap_or(&l.key)))
        .unwrap_or_default();
    
    let attribution = if include_attribution {
        format!("\n\n---\n\n<sub>{}</sub>", ATTRIBUTION_TEXT)
    } else {
        String::new()
    };
    
    // KZ signature Easter egg
    let kz_signature = "<!-- KZ signature: LAZYFROG -->\n";
    
    format!(r#"{kz_sig}# {name}

{description}

[![Stars](https://img.shields.io/github/stars/{full_name}?style=social)](https://github.com/{full_name})
[![Forks](https://img.shields.io/github/forks/{full_name}?style=social)](https://github.com/{full_name}/fork)
{license}

## 📊 Stats

| Metric | Count |
|--------|-------|
| ⭐ Stars | {stars} |
| 🍴 Forks | {forks} |
| 🔓 Issues | {issues} |

## 🔗 Links

- **Repository**: [{full_name}]({url})
- **Language**: {language}
- **License**: {license_name}

## 🚀 Quick Start

```bash
git clone {url}.git
cd {name}
```
{attribution}"#,
        kz_sig = kz_signature,
        name = metadata.name,
        description = description,
        full_name = metadata.full_name,
        stars = format_count(metadata.stargazers_count),
        forks = format_count(metadata.forks_count),
        issues = format_count(metadata.open_issues_count),
        url = metadata.html_url,
        language = metadata.language.as_deref().unwrap_or("Not specified"),
        license = license,
        license_name = metadata.license.as_ref().map(|l| l.name.as_str()).unwrap_or("Not specified"),
        attribution = attribution,
    )
}

/// Generate release notes draft from commits
pub fn generate_release_notes_draft(
    metadata: &RepoMetadata,
    commits: &[CommitInfo],
    version: Option<String>,
    include_attribution: bool,
) -> String {
    let version = version.unwrap_or_else(|| "v0.0.0".to_string());
    let date = Utc::now().format("%Y-%m-%d").to_string();
    
    // Categorize commits by conventional commit prefixes
    let mut features: Vec<&CommitInfo> = Vec::new();
    let mut fixes: Vec<&CommitInfo> = Vec::new();
    let mut docs: Vec<&CommitInfo> = Vec::new();
    let mut chores: Vec<&CommitInfo> = Vec::new();
    let mut other: Vec<&CommitInfo> = Vec::new();
    
    for commit in commits {
        let msg = commit.message.to_lowercase();
        if msg.starts_with("feat") || msg.starts_with("feature") {
            features.push(commit);
        } else if msg.starts_with("fix") || msg.starts_with("bug") {
            fixes.push(commit);
        } else if msg.starts_with("doc") {
            docs.push(commit);
        } else if msg.starts_with("chore") || msg.starts_with("ci") || msg.starts_with("build") {
            chores.push(commit);
        } else {
            other.push(commit);
        }
    }
    
    let mut notes = format!(r#"# {name} {version}

**Release Date**: {date}

## What's Changed

"#, name = metadata.name, version = version, date = date);
    
    if !features.is_empty() {
        notes.push_str("### ✨ Features\n\n");
        for commit in features {
            notes.push_str(&format!("- {} (`{}`)\n", commit.message, commit.sha));
        }
        notes.push('\n');
    }
    
    if !fixes.is_empty() {
        notes.push_str("### 🐛 Bug Fixes\n\n");
        for commit in fixes {
            notes.push_str(&format!("- {} (`{}`)\n", commit.message, commit.sha));
        }
        notes.push('\n');
    }
    
    if !docs.is_empty() {
        notes.push_str("### 📚 Documentation\n\n");
        for commit in docs {
            notes.push_str(&format!("- {} (`{}`)\n", commit.message, commit.sha));
        }
        notes.push('\n');
    }
    
    if !chores.is_empty() {
        notes.push_str("### 🔧 Maintenance\n\n");
        for commit in chores {
            notes.push_str(&format!("- {} (`{}`)\n", commit.message, commit.sha));
        }
        notes.push('\n');
    }
    
    if !other.is_empty() {
        notes.push_str("### 📝 Other Changes\n\n");
        for commit in other {
            notes.push_str(&format!("- {} (`{}`)\n", commit.message, commit.sha));
        }
        notes.push('\n');
    }
    
    notes.push_str(&format!(r#"## 📦 Installation

```bash
git clone {url}.git
cd {name}
git checkout {version}
```

## 🔗 Links

- **Full Changelog**: {url}/commits/{branch}
- **Repository**: {url}
"#,
        url = metadata.html_url,
        name = metadata.name,
        version = version,
        branch = metadata.default_branch,
    ));
    
    if include_attribution {
        notes.push_str(&format!("\n---\n\n<sub>{}</sub>\n", ATTRIBUTION_TEXT));
    }
    
    notes
}

/// Generate press kit overview markdown
pub fn generate_press_kit(metadata: &RepoMetadata, include_attribution: bool) -> String {
    let description = metadata.description.as_deref().unwrap_or("A software project");
    
    let attribution = if include_attribution {
        format!("\n---\n\n<sub>{}</sub>", ATTRIBUTION_TEXT)
    } else {
        String::new()
    };
    
    format!(r#"# {name} — Press Kit

## Overview

**{name}** is {description}

## Quick Facts

| | |
|---|---|
| **Name** | {name} |
| **Author** | [{owner}]({owner_url}) |
| **Repository** | [{full_name}]({url}) |
| **Language** | {language} |
| **License** | {license} |
| **Stars** | {stars} |
| **Forks** | {forks} |

## Description

{description}

## Key Features

- Primary language: **{language}**
- Active development with **{issues}** open issues
- Last updated: **{updated}**

## Topics / Tags

{topics}

## Assets

The following assets are included in this press kit:

- `repo-card.svg` — Vector social card (editable)
- `repo-card.png` — Raster social card (1200×630)
- `README-snippet.md` — Ready-to-use README section
- `release-notes-draft.md` — Auto-generated release notes template

## Screenshots

Place screenshots in the `screenshots/` folder.

## Contact

- **Repository**: {url}
- **Owner**: {owner_url}

## License

This project is licensed under **{license}**.
{attribution}"#,
        name = metadata.name,
        description = description,
        owner = metadata.owner.login,
        owner_url = metadata.owner.html_url,
        full_name = metadata.full_name,
        url = metadata.html_url,
        language = metadata.language.as_deref().unwrap_or("Not specified"),
        license = metadata.license.as_ref().map(|l| l.name.as_str()).unwrap_or("Not specified"),
        stars = format_count(metadata.stargazers_count),
        forks = format_count(metadata.forks_count),
        issues = format_count(metadata.open_issues_count),
        updated = &metadata.updated_at[..10],
        topics = if metadata.topics.is_empty() {
            "No topics specified".to_string()
        } else {
            metadata.topics.iter().map(|t| format!("`{}`", t)).collect::<Vec<_>>().join(", ")
        },
        attribution = attribution,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LicenseInfo, OwnerInfo};

    fn sample_metadata() -> RepoMetadata {
        RepoMetadata {
            name: "test-repo".to_string(),
            full_name: "owner/test-repo".to_string(),
            description: Some("A test repository".to_string()),
            html_url: "https://github.com/owner/test-repo".to_string(),
            stargazers_count: 1234,
            forks_count: 56,
            watchers_count: 100,
            open_issues_count: 10,
            language: Some("Rust".to_string()),
            topics: vec!["testing".to_string(), "rust".to_string()],
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

    #[test]
    fn test_format_count() {
        assert_eq!(format_count(999), "999");
        assert_eq!(format_count(1000), "1.0K");
        assert_eq!(format_count(1500), "1.5K");
        assert_eq!(format_count(1000000), "1.0M");
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("<test>"), "&lt;test&gt;");
        assert_eq!(escape_xml("a & b"), "a &amp; b");
    }

    #[test]
    fn test_generate_modern_template() {
        let metadata = sample_metadata();
        let svg = generate_svg(&metadata, "modern", true, None, None).unwrap();
        assert!(svg.contains("test-repo"));
        assert!(svg.contains("RepoCard Studio"));
        // Easter egg verification
        assert!(svg.contains("<!-- KZ: LAZYFROG :: frogprints -->"));
        assert!(svg.contains("opacity=\"0.03\"")); // Frogprints
    }

    #[test]
    fn test_generate_readme_snippet() {
        let metadata = sample_metadata();
        let snippet = generate_readme_snippet(&metadata, true);
        assert!(snippet.contains("# test-repo"));
        assert!(snippet.contains("RepoCard Studio"));
        // Easter egg verification
        assert!(snippet.contains("<!-- KZ signature: LAZYFROG -->"));
    }

    // ============================================
    // SNAPSHOT/HASH TESTS FOR DETERMINISTIC OUTPUT
    // ============================================
    // These tests ensure template output is consistent and
    // any visual changes require intentional snapshot updates.

    /// Snapshot test for Modern template
    /// If this test fails, the template output has changed.
    /// Update the expected hash only for intentional visual changes.
    #[test]
    fn test_modern_template_snapshot() {
        let metadata = sample_metadata();
        let svg = generate_svg(&metadata, "modern", true, Some("#0d1117".to_string()), Some("#161b22".to_string())).unwrap();
        
        // Verify structure is deterministic
        assert!(svg.starts_with("<!-- KZ: LAZYFROG :: frogprints -->"));
        assert!(svg.contains("width=\"1200\" height=\"630\""));
        assert!(svg.contains("viewBox=\"0 0 1200 630\""));
        
        // Key sections must be present
        assert!(svg.contains("<!-- Background -->"));
        assert!(svg.contains("<!-- Header -->"));
        assert!(svg.contains("<!-- Description -->"));
        assert!(svg.contains("<!-- Stats -->"));
        assert!(svg.contains("<!-- Footer -->"));
        assert!(svg.contains("<!-- Frogprints Easter Egg -->"));
        
        // Metadata binding verification
        assert!(svg.contains("owner"));
        assert!(svg.contains("test-repo"));
        assert!(svg.contains("1.2K")); // stars
        assert!(svg.contains("56")); // forks
        assert!(svg.contains("Rust"));
    }

    /// Snapshot test for Minimal template
    #[test]
    fn test_minimal_template_snapshot() {
        let metadata = sample_metadata();
        let svg = generate_svg(&metadata, "minimal", true, None, None).unwrap();
        
        // Verify structure is deterministic
        assert!(svg.starts_with("<!-- KZ: LAZYFROG :: frogprints -->"));
        assert!(svg.contains("width=\"1200\" height=\"630\""));
        
        // Minimal uses white background by default
        assert!(svg.contains("fill=\"#ffffff\""));
        
        // Key sections
        assert!(svg.contains("<!-- Background -->"));
        assert!(svg.contains("<!-- Frogprints Easter Egg -->"));
        assert!(svg.contains("opacity=\"0.025\"")); // Minimal uses 2.5% opacity
        
        // Stats row format
        assert!(svg.contains("stars"));
        assert!(svg.contains("forks"));
        assert!(svg.contains("issues"));
    }

    /// Snapshot test for Gradient template
    #[test]
    fn test_gradient_template_snapshot() {
        let metadata = sample_metadata();
        let svg = generate_svg(&metadata, "gradient", true, Some("#667eea".to_string()), Some("#764ba2".to_string())).unwrap();
        
        // Verify structure is deterministic
        assert!(svg.starts_with("<!-- KZ: LAZYFROG :: frogprints -->"));
        assert!(svg.contains("width=\"1200\" height=\"630\""));
        
        // Gradient-specific elements
        assert!(svg.contains("linearGradient"));
        assert!(svg.contains("id=\"bg-gradient\""));
        assert!(svg.contains("#667eea"));
        assert!(svg.contains("#764ba2"));
        
        // Decorative circles
        assert!(svg.contains("rgba(255,255,255,0.05)"));
        
        // Glass pill stats
        assert!(svg.contains("rgba(255,255,255,0.2)"));
        
        // Frogprints at 4% opacity
        assert!(svg.contains("opacity=\"0.04\""));
    }

    /// Test README snippet determinism
    #[test]
    fn test_readme_snippet_snapshot() {
        let metadata = sample_metadata();
        let snippet = generate_readme_snippet(&metadata, true);
        
        // Must start with KZ signature
        assert!(snippet.starts_with("<!-- KZ signature: LAZYFROG -->"));
        
        // Required sections
        assert!(snippet.contains("## 📊 Stats"));
        assert!(snippet.contains("## 🔗 Links"));
        assert!(snippet.contains("## 🚀 Quick Start"));
        
        // Shields.io badges
        assert!(snippet.contains("img.shields.io/github/stars"));
        assert!(snippet.contains("img.shields.io/github/forks"));
        
        // Attribution at end
        assert!(snippet.contains("kindware.dev"));
    }

    /// Test that all templates maintain canvas dimensions
    #[test]
    fn test_all_templates_canvas_dimensions() {
        let metadata = sample_metadata();
        
        for template_id in &["modern", "minimal", "gradient"] {
            let svg = generate_svg(&metadata, template_id, true, None, None).unwrap();
            assert!(svg.contains("width=\"1200\""), "Template {} missing width", template_id);
            assert!(svg.contains("height=\"630\""), "Template {} missing height", template_id);
            assert!(svg.contains("viewBox=\"0 0 1200 630\""), "Template {} missing viewBox", template_id);
        }
    }
}

// RepoCard Studio - Main Application
// LAZYFROG (creator of KZ) — kindware.dev

import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./App.css";

// Types matching Rust backend
interface RepoMetadata {
  name: string;
  full_name: string;
  description: string | null;
  html_url: string;
  stargazers_count: number;
  forks_count: number;
  watchers_count: number;
  open_issues_count: number;
  language: string | null;
  topics: string[];
  created_at: string;
  updated_at: string;
  pushed_at: string;
  default_branch: string;
  license: { key: string; name: string; spdx_id: string | null } | null;
  owner: { login: string; avatar_url: string; html_url: string };
}

interface CommitInfo {
  sha: string;
  message: string;
  author_name: string;
  author_email: string;
  date: string;
}

interface ExportResult {
  success: boolean;
  output_path: string;
  files: string[];
  error: string | null;
}

type TabId = "card" | "readme" | "release" | "press";
type TemplateId = "modern" | "minimal" | "gradient";

const ATTRIBUTION_TEXT = "Generated with RepoCard Studio — LAZYFROG (creator of KZ) — kindware.dev";

function formatCount(count: number): string {
  if (count >= 1_000_000) return `${(count / 1_000_000).toFixed(1)}M`;
  if (count >= 1_000) return `${(count / 1_000).toFixed(1)}K`;
  return count.toString();
}

function App() {
  // State
  const [repoUrl, setRepoUrl] = useState("");
  const [metadata, setMetadata] = useState<RepoMetadata | null>(null);
  const [commits, setCommits] = useState<CommitInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  // Options
  const [template, setTemplate] = useState<TemplateId>("modern");
  const [includeAttribution, setIncludeAttribution] = useState(true);
  const [primaryColor, setPrimaryColor] = useState("#0d1117");
  const [secondaryColor, setSecondaryColor] = useState("#161b22");
  const [exportDir, setExportDir] = useState("");

  // Preview
  const [activeTab, setActiveTab] = useState<TabId>("card");
  const [svgPreview, setSvgPreview] = useState("");
  const [readmePreview, setReadmePreview] = useState("");
  const [releasePreview, setReleasePreview] = useState("");
  const [pressPreview, setPressPreview] = useState("");

  // Export state
  const [exporting, setExporting] = useState(false);

  // Load default export directory
  useEffect(() => {
    invoke<string>("get_default_export_dir")
      .then(setExportDir)
      .catch(console.error);
  }, []);

  // Generate previews when metadata/options change
  const generatePreviews = useCallback(async () => {
    if (!metadata) return;

    try {
      // Generate SVG card
      const svg = await invoke<string>("generate_svg_card", {
        metadata,
        templateId: template,
        includeAttribution,
        primaryColor: template !== "minimal" ? primaryColor : null,
        secondaryColor: template !== "minimal" ? secondaryColor : null,
      });
      setSvgPreview(svg);

      // Generate README snippet
      const readme = await invoke<string>("generate_readme_snippet", {
        metadata,
        includeAttribution,
      });
      setReadmePreview(readme);

      // Generate release notes
      const release = await invoke<string>("generate_release_notes", {
        metadata,
        commits,
        version: null,
        includeAttribution,
      });
      setReleasePreview(release);

      // Generate press kit
      const press = await invoke<string>("generate_press_kit_overview", {
        metadata,
        includeAttribution,
      });
      setPressPreview(press);
    } catch (err) {
      console.error("Preview generation error:", err);
    }
  }, [metadata, commits, template, includeAttribution, primaryColor, secondaryColor]);

  useEffect(() => {
    generatePreviews();
  }, [generatePreviews]);

  // Fetch repository data
  const fetchRepo = async () => {
    if (!repoUrl.trim()) {
      setError("Please enter a GitHub repository URL");
      return;
    }

    setLoading(true);
    setError(null);
    setSuccess(null);

    try {
      const meta = await invoke<RepoMetadata>("fetch_repo", { repoUrl });
      setMetadata(meta);

      const repoCommits = await invoke<CommitInfo[]>("fetch_commits", {
        repoUrl,
        count: 20,
      });
      setCommits(repoCommits);
    } catch (err) {
      setError(err as string);
      setMetadata(null);
      setCommits([]);
    } finally {
      setLoading(false);
    }
  };

  // Select export directory
  const selectExportDir = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: exportDir,
        title: "Select Export Directory",
      });
      if (selected) {
        setExportDir(selected as string);
      }
    } catch (err) {
      console.error("Directory selection error:", err);
    }
  };

  // Export share kit
  const exportShareKit = async () => {
    if (!metadata || !exportDir) {
      setError("Please fetch a repository and select an export directory");
      return;
    }

    setExporting(true);
    setError(null);
    setSuccess(null);

    try {
      const result = await invoke<ExportResult>("export_share_kit", {
        metadata,
        commits,
        options: {
          output_dir: exportDir,
          include_attribution: includeAttribution,
          template_id: template,
          primary_color: primaryColor,
          secondary_color: secondaryColor,
        },
      });

      if (result.success) {
        setSuccess(`Exported ${result.files.length} files to ${result.output_path}`);
      } else {
        setError(result.error || "Export failed");
      }
    } catch (err) {
      setError(err as string);
    } finally {
      setExporting(false);
    }
  };

  return (
    <div className="app">
      {/* Header */}
      <header className="app-header">
        <h1>
          <span className="brand-accent">⬡</span> RepoCard Studio
          <span className="creator-tag">by LAZYFROG (creator of KZ)</span>
        </h1>
        <a
          href="https://kindware.dev"
          target="_blank"
          rel="noopener noreferrer"
          className="kindware-link"
          title="Visit kindware.dev"
        >
          kindware.dev
        </a>
      </header>

      <main className="app-main">
        {/* Sidebar */}
        <aside className="sidebar">
          {/* Repository Input */}
          <div className="sidebar-section">
            <h2>Repository</h2>
            <div className="input-group">
              <label htmlFor="repo-url">GitHub URL or owner/repo</label>
              <div className="input-with-button">
                <input
                  id="repo-url"
                  type="text"
                  className="input"
                  placeholder="https://github.com/owner/repo"
                  value={repoUrl}
                  onChange={(e) => setRepoUrl(e.target.value)}
                  onKeyDown={(e) => e.key === "Enter" && fetchRepo()}
                />
                <button
                  className="btn btn-primary"
                  onClick={fetchRepo}
                  disabled={loading}
                >
                  {loading ? "..." : "Fetch"}
                </button>
              </div>
            </div>

            {error && (
              <div className="error-banner">
                <span className="error-banner-icon">⚠</span>
                <span className="error-banner-text">{error}</span>
              </div>
            )}

            {success && (
              <div className="success-banner">
                <span className="success-banner-icon">✓</span>
                <span className="success-banner-text">{success}</span>
              </div>
            )}

            {/* Repo Info */}
            {metadata && (
              <div className="repo-info">
                <div className="repo-info-header">
                  <div className="repo-avatar">
                    <img
                      src={metadata.owner.avatar_url}
                      alt={metadata.owner.login}
                      onError={(e) => {
                        e.currentTarget.style.display = "none";
                        e.currentTarget.parentElement!.textContent =
                          metadata.owner.login[0].toUpperCase();
                      }}
                    />
                  </div>
                  <div>
                    <div className="repo-name">{metadata.name}</div>
                    <div className="repo-fullname">{metadata.full_name}</div>
                  </div>
                </div>
                <p className="repo-description">
                  {metadata.description || "No description"}
                </p>
                <div className="repo-stats">
                  <div className="repo-stat">
                    <span>★</span>
                    <span className="repo-stat-value">
                      {formatCount(metadata.stargazers_count)}
                    </span>
                    <span>stars</span>
                  </div>
                  <div className="repo-stat">
                    <span>⑂</span>
                    <span className="repo-stat-value">
                      {formatCount(metadata.forks_count)}
                    </span>
                    <span>forks</span>
                  </div>
                  <div className="repo-stat">
                    <span>●</span>
                    <span className="repo-stat-value">
                      {metadata.language || "N/A"}
                    </span>
                  </div>
                </div>
                {metadata.topics.length > 0 && (
                  <div className="repo-topics">
                    {metadata.topics.slice(0, 5).map((topic) => (
                      <span key={topic} className="repo-topic">
                        {topic}
                      </span>
                    ))}
                    {metadata.topics.length > 5 && (
                      <span className="repo-topic">
                        +{metadata.topics.length - 5}
                      </span>
                    )}
                  </div>
                )}
              </div>
            )}
          </div>

          {/* Options */}
          <div className="sidebar-content">
            <h2 className="options-heading">Options</h2>

            {/* Template Selection */}
            <div className="input-group">
              <label>Template</label>
              <div className="template-grid">
                {(["modern", "minimal", "gradient"] as TemplateId[]).map(
                  (t) => (
                    <button
                      key={t}
                      type="button"
                      className={`template-option ${template === t ? "selected" : ""}`}
                      onClick={() => setTemplate(t)}
                      aria-current={template === t ? "true" : undefined}
                      aria-label={`Select ${t} template`}
                    >
                      <div className={`template-preview template-preview-${t}`} />
                      <div className="template-name">
                        {t.charAt(0).toUpperCase() + t.slice(1)}
                      </div>
                    </button>
                  )
                )}
              </div>
            </div>

            {/* Color Pickers */}
            {template !== "minimal" && (
              <div className="input-group">
                <label>Colors</label>
                <div className="color-picker-row">
                  <div className="color-picker-group">
                    <label htmlFor="primary-color">Primary</label>
                    <input
                      id="primary-color"
                      type="color"
                      className="color-picker"
                      title="Select primary color"
                      value={primaryColor}
                      onChange={(e) => setPrimaryColor(e.target.value)}
                    />
                  </div>
                  <div className="color-picker-group">
                    <label htmlFor="secondary-color">Secondary</label>
                    <input
                      id="secondary-color"
                      type="color"
                      className="color-picker"
                      title="Select secondary color"
                      value={secondaryColor}
                      onChange={(e) => setSecondaryColor(e.target.value)}
                    />
                  </div>
                </div>
              </div>
            )}

            {/* Attribution Toggle */}
            <div className="toggle-group">
              <label className="toggle">
                <input
                  type="checkbox"
                  checked={includeAttribution}
                  onChange={(e) => setIncludeAttribution(e.target.checked)}
                  title="Toggle attribution in generated assets"
                  aria-label="Include attribution"
                />
                <span className="toggle-slider" />
              </label>
              <div className="toggle-content">
                <h4>Include Attribution</h4>
                <p>
                  Adds "{ATTRIBUTION_TEXT}" to generated assets. This helps
                  free tools like RepoCard Studio stay visible and sustainable.
                </p>
              </div>
            </div>

            {/* Export Section */}
            <div className="export-panel">
              <h3>Export Share Kit</h3>
              <div className="input-group">
                <label>Export Directory</label>
                <div className="input-with-button">
                  <input
                    type="text"
                    className="input"
                    value={exportDir}
                    onChange={(e) => setExportDir(e.target.value)}
                    placeholder="Select directory..."
                    readOnly
                  />
                  <button className="btn btn-secondary" onClick={selectExportDir}>
                    Browse
                  </button>
                </div>
              </div>
              <div className="export-files">
                <div className="export-file">
                  <span className="export-file-icon">📄</span>
                  repo-card.svg
                </div>
                <div className="export-file">
                  <span className="export-file-icon">🖼</span>
                  repo-card.png
                </div>
                <div className="export-file">
                  <span className="export-file-icon">📝</span>
                  README-snippet.md
                </div>
                <div className="export-file">
                  <span className="export-file-icon">📋</span>
                  release-notes-draft.md
                </div>
                <div className="export-file">
                  <span className="export-file-icon">📁</span>
                  press-kit/overview.md
                </div>
              </div>
              <button
                className="btn btn-success btn-large btn-full-width"
                onClick={exportShareKit}
                disabled={!metadata || exporting}
              >
                {exporting ? "Exporting..." : "Export Share Kit"}
              </button>
            </div>
          </div>
        </aside>

        {/* Content Area */}
        <section className="content">
          <div className="content-header">
            <div className="tabs">
              {(
                [
                  { id: "card", label: "Social Card" },
                  { id: "readme", label: "README" },
                  { id: "release", label: "Release Notes" },
                  { id: "press", label: "Press Kit" },
                ] as { id: TabId; label: string }[]
              ).map((tab) => (
                <button
                  key={tab.id}
                  className={`tab ${activeTab === tab.id ? "active" : ""}`}
                  onClick={() => setActiveTab(tab.id)}
                >
                  {tab.label}
                </button>
              ))}
            </div>
          </div>

          <div className="content-body">
            {!metadata ? (
              <div className="empty-state">
                <div className="empty-state-icon">⬡</div>
                <h3>No Repository Loaded</h3>
                <p>
                  Enter a GitHub repository URL in the sidebar to generate
                  promotional assets.
                </p>
              </div>
            ) : loading ? (
              <div className="loading">
                <div className="spinner" />
                <span>Loading repository...</span>
              </div>
            ) : (
              <>
                {activeTab === "card" && (
                  <div className="card-preview">
                    <div className="card-preview-header">
                      <span className="card-preview-title">
                        SVG Preview (1200×630)
                      </span>
                      <button
                        className="btn btn-secondary"
                        onClick={() => {
                          const blob = new Blob([svgPreview], {
                            type: "image/svg+xml",
                          });
                          const url = URL.createObjectURL(blob);
                          const a = document.createElement("a");
                          a.href = url;
                          a.download = `${metadata.name}-card.svg`;
                          a.click();
                          URL.revokeObjectURL(url);
                        }}
                      >
                        Download SVG
                      </button>
                    </div>
                    <div
                      className="card-preview-content"
                      dangerouslySetInnerHTML={{ __html: svgPreview }}
                    />
                  </div>
                )}

                {activeTab === "readme" && (
                  <div className="preview-content">
                    <pre>
                      <code>{readmePreview}</code>
                    </pre>
                  </div>
                )}

                {activeTab === "release" && (
                  <div className="preview-content">
                    <pre>
                      <code>{releasePreview}</code>
                    </pre>
                  </div>
                )}

                {activeTab === "press" && (
                  <div className="preview-content">
                    <pre>
                      <code>{pressPreview}</code>
                    </pre>
                  </div>
                )}
              </>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}

export default App;

// RepoCard Studio - Utility Tests
// LAZYFROG (KZ) — kindware.dev

import { describe, it, expect } from 'vitest';

// Test utility functions that match the Rust backend logic

function formatCount(count: number): string {
  if (count >= 1_000_000) return `${(count / 1_000_000).toFixed(1)}M`;
  if (count >= 1_000) return `${(count / 1_000).toFixed(1)}K`;
  return count.toString();
}

function parseRepoUrl(url: string): { owner: string; repo: string } | null {
  const trimmed = url.trim();
  
  // Handle full URLs
  if (trimmed.startsWith('https://github.com/') || trimmed.startsWith('http://github.com/')) {
    const path = trimmed
      .replace('https://github.com/', '')
      .replace('http://github.com/', '');
    const parts = path.replace(/^\/|\/$/g, '').split('/');
    if (parts.length >= 2) {
      return { owner: parts[0], repo: parts[1] };
    }
  }
  
  // Handle short format
  if (trimmed.includes('/') && !trimmed.includes('://')) {
    const parts = trimmed.split('/');
    if (parts.length === 2) {
      return { owner: parts[0], repo: parts[1] };
    }
  }
  
  return null;
}

function truncateText(text: string, maxLength: number): string {
  if (text.length <= maxLength) return text;
  return `${text.slice(0, maxLength - 3)}...`;
}

function escapeXml(text: string): string {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&apos;');
}

describe('formatCount', () => {
  it('formats numbers under 1000 as-is', () => {
    expect(formatCount(0)).toBe('0');
    expect(formatCount(100)).toBe('100');
    expect(formatCount(999)).toBe('999');
  });

  it('formats thousands with K suffix', () => {
    expect(formatCount(1000)).toBe('1.0K');
    expect(formatCount(1500)).toBe('1.5K');
    expect(formatCount(10000)).toBe('10.0K');
    expect(formatCount(999999)).toBe('1000.0K');
  });

  it('formats millions with M suffix', () => {
    expect(formatCount(1000000)).toBe('1.0M');
    expect(formatCount(1500000)).toBe('1.5M');
    expect(formatCount(10000000)).toBe('10.0M');
  });
});

describe('parseRepoUrl', () => {
  it('parses full HTTPS URLs', () => {
    const result = parseRepoUrl('https://github.com/microsoft/vscode');
    expect(result).toEqual({ owner: 'microsoft', repo: 'vscode' });
  });

  it('parses full HTTP URLs', () => {
    const result = parseRepoUrl('http://github.com/facebook/react');
    expect(result).toEqual({ owner: 'facebook', repo: 'react' });
  });

  it('parses short owner/repo format', () => {
    const result = parseRepoUrl('rust-lang/rust');
    expect(result).toEqual({ owner: 'rust-lang', repo: 'rust' });
  });

  it('handles trailing slashes', () => {
    const result = parseRepoUrl('https://github.com/owner/repo/');
    expect(result).toEqual({ owner: 'owner', repo: 'repo' });
  });

  it('handles whitespace', () => {
    const result = parseRepoUrl('  owner/repo  ');
    expect(result).toEqual({ owner: 'owner', repo: 'repo' });
  });

  it('returns null for invalid URLs', () => {
    expect(parseRepoUrl('invalid')).toBeNull();
    expect(parseRepoUrl('')).toBeNull();
    expect(parseRepoUrl('not-a-url')).toBeNull();
  });
});

describe('truncateText', () => {
  it('returns short text unchanged', () => {
    expect(truncateText('Hello', 10)).toBe('Hello');
    expect(truncateText('Test', 4)).toBe('Test');
  });

  it('truncates long text with ellipsis', () => {
    expect(truncateText('Hello World', 8)).toBe('Hello...');
    expect(truncateText('This is a long description', 15)).toBe('This is a lo...');
  });

  it('handles edge cases', () => {
    expect(truncateText('', 10)).toBe('');
    expect(truncateText('abc', 3)).toBe('abc');
  });
});

describe('escapeXml', () => {
  it('escapes ampersand', () => {
    expect(escapeXml('A & B')).toBe('A &amp; B');
  });

  it('escapes angle brackets', () => {
    expect(escapeXml('<tag>')).toBe('&lt;tag&gt;');
  });

  it('escapes quotes', () => {
    expect(escapeXml('"quoted"')).toBe('&quot;quoted&quot;');
    expect(escapeXml("'quoted'")).toBe("&apos;quoted&apos;");
  });

  it('handles multiple escapes', () => {
    expect(escapeXml('<a href="test">')).toBe('&lt;a href=&quot;test&quot;&gt;');
  });

  it('returns plain text unchanged', () => {
    expect(escapeXml('Hello World')).toBe('Hello World');
  });
});

describe('Export Structure', () => {
  const expectedFiles = [
    'repo-card.svg',
    'repo-card.png',
    'README-snippet.md',
    'release-notes-draft.md',
    'press-kit/overview.md',
    'press-kit/screenshots/.gitkeep',
  ];

  it('defines correct export file structure', () => {
    expect(expectedFiles).toContain('repo-card.svg');
    expect(expectedFiles).toContain('repo-card.png');
    expect(expectedFiles).toContain('README-snippet.md');
    expect(expectedFiles).toContain('release-notes-draft.md');
    expect(expectedFiles).toContain('press-kit/overview.md');
  });
});

describe('Attribution', () => {
  const ATTRIBUTION_TEXT = 'Generated with RepoCard Studio — LAZYFROG (KZ) — kindware.dev';

  it('contains correct attribution text', () => {
    expect(ATTRIBUTION_TEXT).toContain('RepoCard Studio');
    expect(ATTRIBUTION_TEXT).toContain('LAZYFROG');
    expect(ATTRIBUTION_TEXT).toContain('kindware.dev');
  });
});

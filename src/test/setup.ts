// RepoCard Studio - Test Setup
// LAZYFROG (creator of KZ) — kindware.dev

import '@testing-library/jest-dom';
import { vi } from 'vitest';

// Create mock functions
export const mockInvoke = vi.fn().mockImplementation((cmd: string) => {
  switch (cmd) {
    case 'get_default_export_dir':
      return Promise.resolve('/home/user/Downloads');
    case 'fetch_repo':
      return Promise.resolve({
        name: 'test-repo',
        full_name: 'owner/test-repo',
        description: 'A test repository',
        html_url: 'https://github.com/owner/test-repo',
        stargazers_count: 100,
        forks_count: 10,
        watchers_count: 50,
        open_issues_count: 5,
        language: 'TypeScript',
        topics: ['test'],
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-06-01T00:00:00Z',
        pushed_at: '2024-06-01T00:00:00Z',
        default_branch: 'main',
        license: { key: 'mit', name: 'MIT License', spdx_id: 'MIT' },
        owner: { login: 'owner', avatar_url: 'https://github.com/owner.png', html_url: 'https://github.com/owner' },
      });
    case 'fetch_commits':
      return Promise.resolve([]);
    case 'generate_svg_card':
      return Promise.resolve('<svg>Test</svg>');
    case 'generate_readme_snippet':
      return Promise.resolve('# Test');
    case 'generate_release_notes':
      return Promise.resolve('# Release');
    case 'generate_press_kit_overview':
      return Promise.resolve('# Press');
    default:
      return Promise.resolve(null);
  }
});

export const mockOpen = vi.fn().mockResolvedValue('/test/path');

// Mock Tauri APIs
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke,
}));

vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: mockOpen,
}));

// Reset mocks before each test
beforeEach(() => {
  mockInvoke.mockClear();
  mockOpen.mockClear();
});

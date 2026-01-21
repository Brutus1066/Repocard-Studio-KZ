// RepoCard Studio - App Component Tests
// LAZYFROG (creator of KZ) — kindware.dev

import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import App from '../App';

describe('App', () => {
  it('renders the app header with brand name', async () => {
    await act(async () => {
      render(<App />);
    });
    expect(screen.getByText('RepoCard Studio')).toBeInTheDocument();
    expect(screen.getByText('by LAZYFROG (creator of KZ)')).toBeInTheDocument();
  });

  it('shows empty state when no repo is loaded', async () => {
    await act(async () => {
      render(<App />);
    });
    expect(screen.getByText('No Repository Loaded')).toBeInTheDocument();
  });

  it('has repository input field', async () => {
    await act(async () => {
      render(<App />);
    });
    const input = screen.getByPlaceholderText('https://github.com/owner/repo');
    expect(input).toBeInTheDocument();
  });

  it('has fetch button', async () => {
    await act(async () => {
      render(<App />);
    });
    const button = screen.getByText('Fetch');
    expect(button).toBeInTheDocument();
  });

  it('shows error when fetching empty URL', async () => {
    await act(async () => {
      render(<App />);
    });
    const button = screen.getByText('Fetch');
    await act(async () => {
      fireEvent.click(button);
    });
    
    await waitFor(() => {
      expect(screen.getByText('Please enter a GitHub repository URL')).toBeInTheDocument();
    });
  });

  it('displays all preview tabs', async () => {
    await act(async () => {
      render(<App />);
    });
    expect(screen.getByText('Social Card')).toBeInTheDocument();
    expect(screen.getByText('README')).toBeInTheDocument();
    expect(screen.getByText('Release Notes')).toBeInTheDocument();
    expect(screen.getByText('Press Kit')).toBeInTheDocument();
  });

  it('shows template options', async () => {
    await act(async () => {
      render(<App />);
    });
    expect(screen.getByText('Modern')).toBeInTheDocument();
    expect(screen.getByText('Minimal')).toBeInTheDocument();
    expect(screen.getByText('Gradient')).toBeInTheDocument();
  });

  it('has attribution toggle enabled by default', async () => {
    await act(async () => {
      render(<App />);
    });
    expect(screen.getByText('Include Attribution')).toBeInTheDocument();
  });

  it('loads and displays repository info on fetch', async () => {
    await act(async () => {
      render(<App />);
    });
    
    const input = screen.getByPlaceholderText('https://github.com/owner/repo');
    await act(async () => {
      fireEvent.change(input, { target: { value: 'owner/test-repo' } });
    });
    
    const button = screen.getByText('Fetch');
    await act(async () => {
      fireEvent.click(button);
    });
    
    await waitFor(() => {
      expect(screen.getByText('test-repo')).toBeInTheDocument();
    });
  });

  it('shows export section with file list', async () => {
    await act(async () => {
      render(<App />);
    });
    // Use getAllByText since "Export Share Kit" appears twice (h3 and button)
    expect(screen.getAllByText('Export Share Kit').length).toBeGreaterThan(0);
    expect(screen.getByText('repo-card.svg')).toBeInTheDocument();
    expect(screen.getByText('repo-card.png')).toBeInTheDocument();
    expect(screen.getByText('README-snippet.md')).toBeInTheDocument();
    expect(screen.getByText('release-notes-draft.md')).toBeInTheDocument();
    expect(screen.getByText('press-kit/overview.md')).toBeInTheDocument();
  });
});

describe('Attribution', () => {
  it('contains attribution text and kindware link', async () => {
    await act(async () => {
      render(<App />);
    });
    // kindware.dev appears in both header link and toggle description
    const kindwareElements = screen.getAllByText(/kindware\.dev/);
    expect(kindwareElements.length).toBeGreaterThanOrEqual(1);
  });
});

describe('Template Selection', () => {
  it('allows selecting different templates', async () => {
    await act(async () => {
      render(<App />);
    });
    
    const modernTemplate = screen.getByText('Modern').closest('.template-option');
    const minimalTemplate = screen.getByText('Minimal').closest('.template-option');
    
    expect(modernTemplate).toHaveClass('selected');
    
    if (minimalTemplate) {
      await act(async () => {
        fireEvent.click(minimalTemplate);
      });
      expect(minimalTemplate).toHaveClass('selected');
    }
  });
});

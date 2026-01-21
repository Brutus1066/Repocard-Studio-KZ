# Security Policy

## Overview

RepoCard Studio is a **local-first, read-only tool**. It does not collect, transmit, or store any user data beyond what you explicitly save to your own file system.

## What This App Does

- Fetches **public** repository metadata from the GitHub API
- Generates files locally in directories **you choose**
- Optionally accepts a GitHub token for higher rate limits

## What This App Does NOT Do

| Concern | Status |
|---------|--------|
| Telemetry | ❌ None |
| Analytics | ❌ None |
| Cloud storage | ❌ None |
| Background network calls | ❌ None |
| Credential harvesting | ❌ None |

## GitHub Token Handling

If you provide a GitHub Personal Access Token:

- It is used **only** for GitHub API requests
- It is **never** logged, stored, or transmitted elsewhere
- It remains in memory only during the session

## Reporting a Vulnerability

If you discover a security issue, please open a GitHub issue or contact the maintainer directly.

This is a hobby project with no bounty program, but responsible disclosures are appreciated.

## Non-Goals

This project does not aim to:

- Access private repositories (unless you provide a token)
- Store credentials persistently
- Phone home to any server

---

**Built with transparency by LAZYFROG (creator of KZ)** — [kindware.dev](https://kindware.dev)

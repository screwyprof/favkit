# Release Please Automation

## Status

Accepted

## Context

We need an automated way to manage releases that:
- Follows our Conventional Commits practice
- Maintains CHANGELOG.md
- Updates version numbers
- Creates GitHub releases
- Minimizes manual intervention

## Decision

Use Release Please for automated release management because:
- Native support for Conventional Commits
- Built-in Rust support
- Automated changelog generation
- GitHub release automation
- PR-based workflow for release reviews

## Consequences

### Positive
- Automated version bumps based on commit types
- Consistent changelog format
- Release PRs for review
- Reduced manual release work
- Better change tracking

### Negative
- Requires strict commit message format
- Additional GitHub Actions workflow
- Release PRs need manual approval
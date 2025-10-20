# Publishing to crates.io

This repository is set up to automatically publish to crates.io when a version tag is pushed.

## Setup

### 1. Get your crates.io API token

1. Log in to [crates.io](https://crates.io/)
2. Go to Account Settings
3. Generate a new API token

### 2. Add the token to GitHub Secrets

1. Go to your repository on GitHub
2. Navigate to Settings > Secrets and variables > Actions
3. Click "New repository secret"
4. Name: `CRATES_IO_TOKEN`
5. Value: Paste your API token from crates.io
6. Click "Add secret"

## Publishing a new version

### 1. Update the version in Cargo.toml

```toml
[package]
version = "0.2.0"  # Update this
```

### 2. Commit the changes

```bash
git add Cargo.toml
git commit -m "chore: bump version to 0.2.0"
```

### 3. Create and push a tag

```bash
# Create a tag matching the version
git tag v0.2.0

# Push the commit and tag
git push origin main
git push origin v0.2.0
```

### 4. Automatic publishing

The GitHub Action will automatically:
- Run all tests
- Publish to crates.io if tests pass

You can monitor the progress in the "Actions" tab of your GitHub repository.

## Workflow files

- `.github/workflows/publish.yml` - Automatic publishing on version tags
- `.github/workflows/ci.yml` - Continuous integration (tests, clippy, formatting)

## Manual publishing

If you need to publish manually:

```bash
cargo publish --token YOUR_CRATES_IO_TOKEN
```

Or if you have the token in your environment:

```bash
cargo login
cargo publish
```

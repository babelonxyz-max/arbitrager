# Push to GitHub - Quick Guide

The repo is created! Here's how to push:

## Option 1: Use GitHub CLI (Easiest)

```bash
gh auth login
git push -u origin main
```

## Option 2: Configure Git Credentials

```bash
# Configure credential helper
git config --global credential.helper osxkeychain

# Then push
git push -u origin main
```

You'll be prompted for your GitHub username and password/token.

## Option 3: Use Personal Access Token

1. Create a token at: https://github.com/settings/tokens
2. Use it as password when prompted:
```bash
git push -u origin main
# Username: babeloxyz
# Password: [paste your token]
```

## Option 4: Use SSH (If you have SSH keys)

```bash
git remote set-url origin git@github.com:babeloxyz/arbitrager.git
git push -u origin main
```

## Quick Command

Just run:
```bash
git push -u origin main
```

If it asks for credentials, use your GitHub username and a Personal Access Token as the password.

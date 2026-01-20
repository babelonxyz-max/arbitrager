# Quick Setup: Create Repo and Deploy

Since I need GitHub authentication to create the repo, here's the fastest path:

## Option 1: Create Repo via GitHub Web UI (2 minutes)

1. **Go to**: https://github.com/new
2. **Repository name**: `arbitrager`
3. **Visibility**: Public or Private
4. **Don't** initialize with README, .gitignore, or license
5. **Click**: "Create repository"

Then run:
```bash
cd /Users/mark/arbitrager
git push -u origin main
```

## Option 2: Use GitHub CLI (if you have it)

```bash
gh repo create babeloxyz/arbitrager --public --source=. --remote=origin --push
```

## After Repo is Created

Once the repo exists, I can:
1. Push all code
2. Set up deployment to Render/Railway
3. Connect dashboard

Let me know when the repo is created and I'll push everything!

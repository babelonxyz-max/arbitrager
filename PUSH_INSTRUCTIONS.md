# Push Code to GitHub

The repository is named `solarb`. Here's how to push:

## Quick Push

Run this script:
```bash
./push-to-github.sh
```

## Manual Push

If the script doesn't work, try:

```bash
# Make sure remote is correct
git remote set-url origin https://github.com/babeloxyz/solarb.git

# Push
git push -u origin main
```

If you get authentication errors, you may need to:

1. **Use GitHub CLI**:
```bash
gh auth login
git push -u origin main
```

2. **Or configure git credential helper**:
```bash
git config --global credential.helper osxkeychain
git push -u origin main
```

3. **Or use SSH** (if you have SSH keys set up):
```bash
git remote set-url origin git@github.com:babeloxyz/solarb.git
git push -u origin main
```

## After Push

Once code is on GitHub:

1. **Deploy Bot**: Go to https://dashboard.render.com → New Web Service → Connect GitHub → Select `solarb` repo
2. **Connect Dashboard**: Set `NEXT_PUBLIC_BOT_API_URL` in Vercel

## Current Status

- ✅ Remote configured: `https://github.com/babeloxyz/solarb.git`
- ✅ 5 commits ready
- ✅ 70 files ready to push
- ⏳ Waiting for push to complete

# Troubleshooting: Dashboard Not Connecting

## Common Issues and Solutions

### 1. Bot Server Not Running

**Symptom**: Dashboard shows "Connecting to bot..." or connection errors

**Solution**:
```bash
# Check if bot is running
lsof -i :8080

# If not running, start it:
cd /Users/mark/arbitrager
cargo run --bin arb-daemon

# Or build and run:
cargo build --release --bin arb-daemon
./target/release/arb-daemon
```

### 2. Config File Missing

**Symptom**: Bot fails to start with "Failed to read config"

**Solution**:
```bash
cd /Users/mark/arbitrager
cp config/example.toml config/local.toml
# Edit config/local.toml with your API keys (optional for dry-run mode)
```

### 3. Port Already in Use

**Symptom**: "Address already in use" error

**Solution**:
```bash
# Find what's using port 8080
lsof -i :8080

# Kill the process or change port in config
```

### 4. Environment Variable Not Set

**Symptom**: Dashboard connects to wrong URL

**Solution**:
- **Local**: Create `dashboard/.env.local`:
  ```
  NEXT_PUBLIC_BOT_API_URL=http://localhost:8080
  ```

- **Vercel**: Go to project settings → Environment Variables
  - Add: `NEXT_PUBLIC_BOT_API_URL`
  - Value: Your bot server URL
  - Redeploy after setting

### 5. CORS Issues

**Symptom**: Browser console shows CORS errors

**Solution**: Bot API includes CORS middleware. If issues persist:
- Ensure bot is accessible from dashboard domain
- Check firewall/security groups allow connections
- Verify bot URL is correct

### 6. Bot API Not Responding

**Test the API directly**:
```bash
# Check if bot API is responding
curl http://localhost:8080/api/status

# Should return JSON like:
# {"status":"running","strategies":{...},"kill_switch_active":false,...}
```

### 7. Compilation Errors

**Symptom**: Bot won't compile

**Solution**:
```bash
# Clean and rebuild
cargo clean
cargo build --bin arb-daemon

# Check for missing dependencies
cargo check
```

### 8. Strategies Not Enabled

**Symptom**: Bot starts but shows "No strategies enabled"

**Solution**: Edit `config/local.toml`:
```toml
[strategies]
funding_arb_enabled = true
hyperevm_spot_enabled = true
solana_jupiter_enabled = true
```

## Quick Diagnostic Commands

```bash
# 1. Check if bot is running
lsof -i :8080

# 2. Test API endpoint
curl http://localhost:8080/api/status

# 3. Check config exists
test -f config/local.toml && echo "OK" || echo "Missing"

# 4. Check dashboard env var (local)
cat dashboard/.env.local 2>/dev/null || echo "Not set"

# 5. View bot logs
# (if running in terminal, check output)
```

## Step-by-Step Debugging

1. **Start the bot server**:
   ```bash
   cd /Users/mark/arbitrager
   cargo run --bin arb-daemon
   ```
   Look for: "Starting API server on http://0.0.0.0:8080"

2. **Test API in another terminal**:
   ```bash
   curl http://localhost:8080/api/status
   ```
   Should return JSON response

3. **Check dashboard environment**:
   - Local: `cat dashboard/.env.local`
   - Vercel: Check project settings

4. **Check browser console**:
   - Open browser dev tools (F12)
   - Look for network errors or CORS issues
   - Check if requests are going to correct URL

5. **Verify network connectivity**:
   - If bot is on different server, ensure it's accessible
   - Check firewall rules
   - Test with curl from dashboard server

## Still Not Working?

1. Check bot logs for errors
2. Check browser console for errors
3. Verify all environment variables are set
4. Ensure bot server is accessible from dashboard location
5. Try accessing bot API directly in browser: `http://your-bot-url/api/status`

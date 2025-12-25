# Vendor ESM Files - Simple Commands

## Quickest Method (3 commands)

```bash
# 1. Setup
mkdir tinymce-vendor && cd tinymce-vendor && mkdir vendor

# 2. Download ESM file
curl -o vendor/tinymce.js "https://esm.sh/tinymce@7"

# 3. Create import map
echo '{"imports":{"tinymce":"./vendor/tinymce.js"}}' > importmap.json
```

**That's it!** You now have:
- ✅ `vendor/tinymce.js` - The ESM JavaScript file
- ✅ `importmap.json` - Import map pointing to local file

## Alternative: Using wget

```bash
mkdir tinymce-vendor && cd tinymce-vendor && mkdir vendor
wget -O vendor/tinymce.js https://esm.sh/tinymce@7
echo '{"imports":{"tinymce":"./vendor/tinymce.js"}}' > importmap.json
```

## What You Get

**importmap.json:**
```json
{
  "imports": {
    "tinymce": "./vendor/tinymce.js"
  }
}
```

**vendor/tinymce.js:**
- Actual ESM JavaScript file (not a CDN link)
- Can use offline
- Full source code vendored locally

## Use It

```javascript
// app.js
import tinymce from 'tinymce';

tinymce.init({
  selector: '#editor'
});
```

## Different CDN Sources

### esm.sh (Pure ESM, optimized)
```bash
curl -o vendor/tinymce.js "https://esm.sh/tinymce@7"
```

### jsdelivr (+esm endpoint)
```bash
curl -o vendor/tinymce.js "https://cdn.jsdelivr.net/npm/tinymce@7/+esm"
```

### unpkg (with ?module)
```bash
curl -o vendor/tinymce.js "https://unpkg.com/tinymce@7?module"
```

### jspm.io
```bash
curl -o vendor/tinymce.js "https://ga.jspm.io/npm:tinymce@7.7.0/tinymce.min.js"
```

## Verify ESM Format

```bash
# Check if it's real ESM (should see export/import keywords)
head -100 vendor/tinymce.js | grep -E "export|import"
```

## Complete Example

```bash
# Full workflow
mkdir my-project && cd my-project

# Create folder structure
mkdir vendor

# Download TinyMCE ESM
curl -o vendor/tinymce.js "https://esm.sh/tinymce@7"

# Create import map
cat > importmap.json << 'EOF'
{
  "imports": {
    "tinymce": "./vendor/tinymce.js"
  }
}
EOF

# Create your app
cat > app.js << 'EOF'
import tinymce from 'tinymce';
console.log('Loaded:', tinymce);
EOF

# Done! Files ready to use
ls -la
```

## File Structure

```
my-project/
├── vendor/
│   └── tinymce.js          ← ESM JavaScript file (vendored)
├── importmap.json          ← Import map (points to ./vendor/)
└── app.js                  ← Your code (imports from 'tinymce')
```

## Key Differences from CDN

| CDN Approach | Vendored Approach |
|-------------|------------------|
| `"tinymce": "https://cdn.com/..."` | `"tinymce": "./vendor/tinymce.js"` |
| Files on remote server | Files in your project |
| Requires internet | Works offline |
| Fast (CDN cache) | Fast (local disk) |

## Serve and Test

```bash
# Simple server
python3 -m http.server 8000

# Or use Node
npx http-server -p 8000

# Or Deno with import map support
deno run --import-map=importmap.json app.js
```

---

## Summary

**3 files total:**
1. `vendor/tinymce.js` - ESM source code (downloaded)
2. `importmap.json` - Maps `tinymce` → `./vendor/tinymce.js`
3. Your app code - Uses `import tinymce from 'tinymce'`

**2 commands:**
```bash
curl -o vendor/tinymce.js "https://esm.sh/tinymce@7"
echo '{"imports":{"tinymce":"./vendor/tinymce.js"}}' > importmap.json
```

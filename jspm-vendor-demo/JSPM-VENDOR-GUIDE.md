# JSPM Vendor Guide - Exact Commands

## How to Vendor ESM Files with JSPM

### The Commands (3 steps)

```bash
# 1. Create package.json with dependency
echo '{"name":"my-project","type":"module","dependencies":{"tinymce":"^7.0.0"}}' > package.json

# 2. Install package to get local files
npm install

# 3. Use JSPM to create import map pointing to local files
npx jspm link tinymce --provider nodemodules --out importmap.json
```

### What You Get

**File Structure:**
```
my-project/
├── node_modules/
│   └── tinymce/
│       ├── tinymce.js           ← ESM file (1.7MB)
│       ├── tinymce.min.js       ← Minified version
│       ├── plugins/             ← All plugins
│       ├── themes/              ← Themes
│       └── icons/               ← Icons
├── importmap.json               ← Import map
└── package.json
```

**importmap.json:**
```json
{
  "imports": {
    "tinymce": "./node_modules/tinymce/tinymce.js"
  }
}
```

### Why This Works

1. **npm install** downloads the actual ESM files to `node_modules/`
2. **jspm link --provider nodemodules** creates an import map pointing to those local files
3. You now have:
   - ✅ Import map (`importmap.json`)
   - ✅ ESM JS files (in `node_modules/tinymce/`)
   - ✅ Everything local (no CDN)
   - ✅ Works offline

### Use in Your Code

```javascript
// app.js
import tinymce from 'tinymce';

tinymce.init({
  selector: '#editor'
});
```

### Alternative: Generate JS Injection Script

```bash
# Instead of JSON, generate importmap.js
npx jspm link tinymce --provider nodemodules --out importmap.js
```

**Result (importmap.js):**
```javascript
(map => {
  document.head.appendChild(Object.assign(document.createElement("script"), {
    type: "importmap",
    innerHTML: JSON.stringify({ imports: map.imports })
  }));
})({
  "imports": {
    "tinymce": "./node_modules/tinymce/tinymce.js"
  }
});
```

### Complete Example

```bash
# Full workflow
mkdir my-tinymce-project
cd my-tinymce-project

# Setup package.json with tinymce dependency
cat > package.json << 'EOF'
{
  "name": "my-tinymce-project",
  "type": "module",
  "dependencies": {
    "tinymce": "^7.0.0"
  }
}
EOF

# Install tinymce (downloads ESM files to node_modules)
npm install

# Generate import map pointing to local files
npx jspm link tinymce --provider nodemodules --out importmap.json

# Verify
ls node_modules/tinymce/tinymce.js  # Should exist
cat importmap.json                   # Should point to ./node_modules/
```

### Different Output Formats

```bash
# JSON format
npx jspm link tinymce --provider nodemodules --out importmap.json

# JS injection script format
npx jspm link tinymce --provider nodemodules --out importmap.js

# Print to stdout
npx jspm link tinymce --provider nodemodules --stdout
```

### Multiple Packages

```bash
# Add multiple dependencies to package.json
cat > package.json << 'EOF'
{
  "name": "multi-package",
  "type": "module",
  "dependencies": {
    "tinymce": "^7.0.0",
    "lodash-es": "^4.17.21",
    "date-fns": "^2.30.0"
  }
}
EOF

# Install all
npm install

# Generate import map for all
npx jspm link tinymce lodash-es date-fns --provider nodemodules --out importmap.json
```

**Result:**
```json
{
  "imports": {
    "tinymce": "./node_modules/tinymce/tinymce.js",
    "lodash-es": "./node_modules/lodash-es/lodash.js",
    "date-fns": "./node_modules/date-fns/index.js"
  }
}
```

### Production Optimizations

```bash
# Generate production import map
npx jspm link tinymce \
  --provider nodemodules \
  --out importmap.json \
  --release \
  --flatten-scopes \
  --combine-subpaths
```

### Key Points

1. **`--provider nodemodules`** tells JSPM to use local node_modules instead of CDN
2. **`npm install`** downloads the actual files (vendoring)
3. **Import map points to `./node_modules/`** (local paths, not URLs)
4. **Everything is offline-ready**

### Verify It's Working

```bash
# Check import map points to local files
cat importmap.json | grep node_modules

# Check actual file exists
ls -lh node_modules/tinymce/tinymce.js

# File should be ~1.7MB
```

### Summary

**3 Commands:**
```bash
echo '{"name":"project","type":"module","dependencies":{"tinymce":"^7.0.0"}}' > package.json
npm install
npx jspm link tinymce --provider nodemodules --out importmap.json
```

**Result:**
- `node_modules/tinymce/` ← ESM files (vendored)
- `importmap.json` ← Points to local files

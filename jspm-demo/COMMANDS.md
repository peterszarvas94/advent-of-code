# Exact JSPM Commands for TinyMCE

## Quick Start (Copy & Paste)

### Method 1: Simple Setup (Recommended)

```bash
# 1. Create project directory
mkdir tinymce-jspm-demo
cd tinymce-jspm-demo

# 2. Initialize package.json
echo '{"name":"tinymce-demo","type":"module"}' > package.json

# 3. Link TinyMCE and generate import map
npx jspm link tinymce

# 4. Create HTML file
cat > index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
  <script src="importmap.js"></script>
</head>
<body>
  <textarea id="editor"></textarea>
  <script type="module">
    import tinymce from 'tinymce';
    tinymce.init({ selector: '#editor' });
  </script>
</body>
</html>
EOF

# 5. Serve and test
npx http-server -p 8000
```

### Method 2: All-in-One HTML

```bash
# 1. Setup
mkdir tinymce-demo && cd tinymce-demo

# 2. Create initial HTML
cat > index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
  <title>TinyMCE</title>
</head>
<body>
  <textarea id="editor"></textarea>
  <script type="module">
    import tinymce from 'tinymce';
    tinymce.init({ selector: '#editor' });
  </script>
</body>
</html>
EOF

# 3. Link and inject directly into HTML
npx jspm link index.html

# 4. Serve
npx http-server -p 8000
```

### Method 3: Production Ready

```bash
# 1. Setup
mkdir tinymce-prod && cd tinymce-prod

# 2. Create HTML
cat > index.html << 'EOF'
<!DOCTYPE html>
<html>
<head><title>TinyMCE</title></head>
<body>
  <textarea id="editor"></textarea>
  <script type="module">
    import tinymce from 'tinymce';
    tinymce.init({ selector: '#editor' });
  </script>
</body>
</html>
EOF

# 3. Link with production optimizations + security
npx jspm link index.html --release --integrity --preload

# 4. Serve
npx http-server -p 8000
```

## Step-by-Step Explanation

### Command Breakdown

#### 1. Link Package
```bash
npx jspm link tinymce
```
**Output**: Creates `importmap.js` with TinyMCE ESM URLs

#### 2. Link to JSON
```bash
npx jspm link tinymce --out importmap.json
```
**Output**: Creates `importmap.json` instead

#### 3. Link to HTML
```bash
npx jspm link tinymce --out index.html
```
**Output**: Injects import map into `index.html`

#### 4. Link Local File
```bash
npx jspm link ./src/app.js
```
**Output**: Scans app.js imports and generates map

#### 5. Link HTML File
```bash
npx jspm link index.html
```
**Output**: Scans all `<script type="module">` in HTML

## Common Command Patterns

### Basic Usage
```bash
npx jspm link <package-name>
npx jspm link tinymce
npx jspm link react react-dom
npx jspm link lodash-es
```

### With Options
```bash
# Different CDN provider
npx jspm link tinymce --provider jsdelivr
npx jspm link tinymce --provider unpkg

# Output formats
npx jspm link tinymce --out importmap.js    # JS injection script
npx jspm link tinymce --out importmap.json  # JSON only
npx jspm link tinymce --out index.html      # Embed in HTML

# Production mode
npx jspm link tinymce --release

# Security (add SRI hashes)
npx jspm link tinymce --integrity

# Performance (add preload tags)
npx jspm link tinymce --out index.html --preload

# Combine all
npx jspm link tinymce --out index.html --release --integrity --preload
```

### Update Commands
```bash
# Update all packages
npx jspm update

# Update specific package
npx jspm update tinymce

# Clear cache
npx jspm clear-cache
```

### Inspect Commands
```bash
# List package exports
npx jspm ls tinymce

# View configuration
npx jspm config list
```

## Generated Import Map Examples

### Default Output (importmap.js)
```javascript
(map => {
  document.head.appendChild(Object.assign(document.createElement("script"), {
    type: "importmap",
    innerHTML: JSON.stringify({ imports: map.imports })
  }));
})({
  "imports": {
    "tinymce": "https://ga.jspm.io/npm:tinymce@7.7.0/tinymce.min.js"
  }
});
```

### JSON Output (importmap.json)
```json
{
  "imports": {
    "tinymce": "https://ga.jspm.io/npm:tinymce@7.7.0/tinymce.min.js"
  }
}
```

### HTML Output (embedded)
```html
<!DOCTYPE html>
<html>
<head>
  <script type="importmap">
  {
    "imports": {
      "tinymce": "https://ga.jspm.io/npm:tinymce@7.7.0/tinymce.min.js"
    }
  }
  </script>
</head>
<!-- rest of HTML -->
```

## Complete Working Example

```bash
# Copy and run this entire block
mkdir jspm-tinymce-test && cd jspm-tinymce-test && \
echo '{"name":"test","type":"module"}' > package.json && \
cat > index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>JSPM + TinyMCE</title>
</head>
<body>
  <h1>TinyMCE Editor</h1>
  <textarea id="editor">Edit me!</textarea>
  <script type="module">
    import tinymce from 'tinymce';
    import 'tinymce/icons/default';
    import 'tinymce/themes/silver';
    import 'tinymce/models/dom';

    tinymce.init({
      selector: '#editor',
      height: 500,
      menubar: false,
      plugins: 'lists link image table',
      toolbar: 'undo redo | bold italic | bullist numlist | link image'
    });
  </script>
</body>
</html>
EOF
npx jspm link index.html --preload && \
echo "✅ Setup complete! Run: npx http-server -p 8000"
```

## Troubleshooting Commands

```bash
# If network issues, use alternative CDN
npx jspm link tinymce --provider jsdelivr

# If cache issues
npx jspm clear-cache

# If module resolution issues
npx jspm link tinymce --provider esm.sh

# Debug mode (verbose output)
npx jspm link tinymce --stdout

# Force specific version
npx jspm link tinymce@7.7.0
```

## What Each Command Does

| Command | Action |
|---------|--------|
| `npx jspm link tinymce` | Downloads package info, generates importmap.js |
| `npx jspm link --out index.html` | Embeds import map into HTML file |
| `npx jspm link --provider jsdelivr` | Uses JSDelivr CDN instead of jspm.io |
| `npx jspm link --release` | Production mode (flatten, optimize) |
| `npx jspm link --integrity` | Adds SRI security hashes |
| `npx jspm link --preload` | Adds preload tags for performance |
| `npx jspm update` | Updates packages to latest versions |
| `npx jspm ls tinymce` | Shows package exports/entry points |

## Final Verification

```bash
# After running jspm link, verify files exist:
ls -la

# Should see:
# - importmap.js (or importmap.json)
# - index.html (if created)
# - package.json

# View the import map:
cat importmap.js
# or
cat importmap.json
```

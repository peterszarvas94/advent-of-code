# JSPM CLI Guide - Import Maps & ESM

## What is JSPM?

JSPM (JavaScript Package Manager) is a tool for managing browser-native ES modules using import maps. It allows you to use npm packages directly in the browser without bundling.

## Installation

```bash
# Global installation
npm install -g jspm

# Or use with npx (no installation needed)
npx jspm <command>
```

## Basic Workflow

### 1. Initialize a Project

```bash
# Create a new directory
mkdir my-project
cd my-project

# Initialize package.json
npm init -y

# Or use jspm init
npx jspm init
```

### 2. Link Packages (Generate Import Maps)

```bash
# Link a specific package from npm
npx jspm link tinymce

# Link multiple packages
npx jspm link react react-dom

# Link a local module file
npx jspm link ./src/main.js

# Link an HTML file (scans for all module imports)
npx jspm link index.html
```

### 3. Generated Files

JSPM creates `importmap.js` (default) or `importmap.json` containing:

**importmap.js** (recommended - injection script):
```javascript
(map => {
  document.head.appendChild(Object.assign(document.createElement("script"), {
    type: "importmap",
    innerHTML: JSON.stringify({
      imports: map.imports
    })
  }));
})({
  "imports": {
    "tinymce": "https://ga.jspm.io/npm:tinymce@7.7.0/tinymce.min.js"
  }
});
```

**importmap.json** (raw import map):
```json
{
  "imports": {
    "tinymce": "https://ga.jspm.io/npm:tinymce@7.7.0/tinymce.min.js"
  }
}
```

### 4. Using the Import Map

**Option A: Using importmap.js (recommended)**

```html
<!DOCTYPE html>
<html>
<head>
  <!-- Include the injection script -->
  <script src="importmap.js"></script>
</head>
<body>
  <script type="module">
    // Now you can import directly!
    import tinymce from 'tinymce';
    console.log(tinymce);
  </script>
</body>
</html>
```

**Option B: Direct HTML embedding**

```bash
# Embed directly into HTML file
npx jspm link tinymce --out index.html
```

Result:
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
```

## Common Options

### Output Formats

```bash
# Output as JS injection script (default)
npx jspm link tinymce --out importmap.js

# Output as JSON
npx jspm link tinymce --out importmap.json

# Embed in HTML
npx jspm link tinymce --out index.html

# Output to stdout
npx jspm link tinymce --stdout
```

### CDN Providers

```bash
# Use different CDN providers
npx jspm link tinymce --provider jsdelivr
npx jspm link tinymce --provider unpkg
npx jspm link tinymce --provider esm.sh

# Default is jspm.io (optimized ESM CDN)
```

### Security & Performance

```bash
# Add integrity hashes (Subresource Integrity)
npx jspm link tinymce --integrity

# Add preload tags (in HTML output)
npx jspm link tinymce --out index.html --preload

# Release mode (production optimizations)
npx jspm link tinymce --release
```

### Environment Conditions

```bash
# Specify environment conditions
npx jspm link react --conditions=production,browser

# Development mode (default)
npx jspm link react --conditions=development
```

## TinyMCE Example

### Step 1: Link TinyMCE

```bash
npx jspm link tinymce
```

### Step 2: Create HTML

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>TinyMCE with JSPM</title>
  <script src="importmap.js"></script>
</head>
<body>
  <textarea id="editor"></textarea>

  <script type="module">
    import tinymce from 'tinymce';

    tinymce.init({
      selector: '#editor',
      height: 500,
      plugins: 'lists link image',
      toolbar: 'undo redo | bold italic | bullist numlist'
    });
  </script>
</body>
</html>
```

### Step 3: Serve

```bash
# JSPM includes a dev server
npx jspm serve

# Or use any static server
npx http-server
python -m http.server 8000
```

## Advanced Usage

### Update Packages

```bash
# Update all packages to latest versions
npx jspm update

# Update specific package
npx jspm update tinymce
```

### List Package Exports

```bash
# See what exports a package has
npx jspm ls tinymce
```

### Local Development

```bash
# Install local package.json exports
npx jspm install

# This reads your package.json and creates import map for local development
```

### Working with package.json

```json
{
  "name": "my-app",
  "type": "module",
  "dependencies": {
    "tinymce": "^7.0.0"
  },
  "exports": {
    ".": "./src/index.js"
  }
}
```

```bash
# Install based on package.json
npx jspm install
```

## Benefits of JSPM

1. **No Build Step**: Use npm packages directly in browser
2. **Native ESM**: Leverages browser-native ES modules
3. **CDN Optimization**: Packages served from optimized CDNs
4. **Development Speed**: Instant updates, no rebuild needed
5. **Production Ready**: Can use with integrity checks and preloading

## Common Patterns

### Pattern 1: Quick Prototyping

```bash
npx jspm link lodash-es date-fns
# Start coding immediately with ES modules
```

### Pattern 2: Production Build

```bash
npx jspm link ./src/app.js --release --integrity --out index.html
# Optimized import map with security
```

### Pattern 3: Hybrid Approach

```bash
# Use JSPM for development
npx jspm link react react-dom

# Build with Vite/Webpack for production
npm run build
```

## Troubleshooting

### Network Issues

If jspm.io is unreachable, use alternative providers:
```bash
npx jspm link tinymce --provider jsdelivr
npx jspm link tinymce --provider unpkg
```

### Cache Issues

```bash
# Clear JSPM cache
npx jspm clear-cache
```

### Version Conflicts

```bash
# Lock specific versions in import map
npx jspm link tinymce@7.7.0
```

## Resources

- JSPM Docs: https://jspm.org
- Import Maps Spec: https://github.com/WICG/import-maps
- ES Modules: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules

# JSPM + curl Vendor Workflow

## The Idea

1. **JSPM generates import map** with CDN URLs
2. **curl downloads** those URLs to local files
3. **Update import map** to point to local files

## Quick Commands

```bash
# 1. Generate import map (with CDN URLs)
npx jspm link react --out importmap.json

# 2. Create vendor directory
mkdir vendor

# 3. curl the URLs to vendor/
curl -o vendor/react.js "https://ga.jspm.io/npm:react@19.1.0/index.js"

# 4. Update import map to point to local files
# Change: "react": "https://ga.jspm.io/npm:react@19.1.0/index.js"
# To:     "react": "./vendor/react.js"
```

## Step-by-Step Example with React

### Step 1: Generate Import Map

```bash
mkdir my-project && cd my-project
echo '{"name":"my-project","type":"module"}' > package.json
npx jspm link react --out importmap.json
```

**Result (importmap.json):**
```json
{
  "imports": {
    "react": "https://ga.jspm.io/npm:react@19.1.0/index.js",
    "react/jsx-runtime": "https://ga.jspm.io/npm:react@19.1.0/jsx-runtime.js"
  }
}
```

### Step 2: Download Files with curl

```bash
mkdir vendor

# Download each URL
curl -o vendor/react.js \
  "https://ga.jspm.io/npm:react@19.1.0/index.js"

curl -o vendor/react-jsx-runtime.js \
  "https://ga.jspm.io/npm:react@19.1.0/jsx-runtime.js"
```

### Step 3: Update Import Map

```bash
cat > importmap.json << 'EOF'
{
  "imports": {
    "react": "./vendor/react.js",
    "react/jsx-runtime": "./vendor/react-jsx-runtime.js"
  }
}
EOF
```

### Final Result

**File structure:**
```
my-project/
├── vendor/
│   ├── react.js                 ← Downloaded ESM file
│   └── react-jsx-runtime.js     ← Downloaded ESM file
├── importmap.json               ← Points to ./vendor/
└── package.json
```

**importmap.json:**
```json
{
  "imports": {
    "react": "./vendor/react.js",
    "react/jsx-runtime": "./vendor/react-jsx-runtime.js"
  }
}
```

## Automated Script

### Manual Approach

```bash
# Generate
npx jspm link react --out importmap.json

# Extract URLs manually and curl
grep -Po '"https://[^"]*"' importmap.json | while read -r url; do
  url=$(echo $url | tr -d '"')
  filename=$(basename $url)
  curl -o "vendor/$filename" "$url"
done

# Update import map manually
```

### Using jq (Automated)

```bash
#!/bin/bash
# auto-vendor.sh

# 1. Generate import map
npx jspm link react --out importmap.json

# 2. Create vendor directory
mkdir -p vendor

# 3. Extract and download all URLs
jq -r '.imports | to_entries[] | "\(.key)|\(.value)"' importmap.json | \
while IFS='|' read -r name url; do
  if [[ $url == http* ]]; then
    filename=$(echo "$name" | sed 's/\//-/g').js
    echo "Downloading $name -> vendor/$filename"
    curl -sS -o "vendor/$filename" "$url"

    # Save mapping for later
    echo "$name|./vendor/$filename" >> mappings.txt
  fi
done

# 4. Rebuild import map with local paths
echo '{' > importmap.json
echo '  "imports": {' >> importmap.json

first=true
while IFS='|' read -r name path; do
  [ "$first" = false ] && echo ',' >> importmap.json
  first=false
  echo -n "    \"$name\": \"$path\"" >> importmap.json
done < mappings.txt

echo '' >> importmap.json
echo '  }' >> importmap.json
echo '}' >> importmap.json

rm mappings.txt
```

## Example: TinyMCE

```bash
# 1. Generate
npx jspm link tinymce --out importmap.json

# Output:
# {
#   "imports": {
#     "tinymce": "https://ga.jspm.io/npm:tinymce@7.7.0/tinymce.min.js"
#   }
# }

# 2. curl
mkdir vendor
curl -o vendor/tinymce.js "https://ga.jspm.io/npm:tinymce@7.7.0/tinymce.min.js"

# 3. Update
cat > importmap.json << 'EOF'
{
  "imports": {
    "tinymce": "./vendor/tinymce.js"
  }
}
EOF
```

## Example: Multiple Packages

```bash
# 1. Generate for multiple packages
npx jspm link react react-dom lodash-es --out importmap.json

# Output:
# {
#   "imports": {
#     "react": "https://ga.jspm.io/npm:react@19.1.0/index.js",
#     "react-dom": "https://ga.jspm.io/npm:react-dom@19.1.0/index.js",
#     "lodash-es": "https://ga.jspm.io/npm:lodash-es@4.17.21/lodash.js"
#   }
# }

# 2. Download all
mkdir vendor
curl -o vendor/react.js "https://ga.jspm.io/npm:react@19.1.0/index.js"
curl -o vendor/react-dom.js "https://ga.jspm.io/npm:react-dom@19.1.0/index.js"
curl -o vendor/lodash-es.js "https://ga.jspm.io/npm:lodash-es@4.17.21/lodash.js"

# 3. Update import map
cat > importmap.json << 'EOF'
{
  "imports": {
    "react": "./vendor/react.js",
    "react-dom": "./vendor/react-dom.js",
    "lodash-es": "./vendor/lodash-es.js"
  }
}
EOF
```

## One-Liner Curl Script

```bash
# Extract all URLs and download them
cat importmap.json | \
jq -r '.imports[]' | \
grep '^https://' | \
xargs -I {} sh -c 'curl -o "vendor/$(basename {})" "{}"'
```

## Pros & Cons

### Pros
- ✅ Simple workflow
- ✅ JSPM handles dependency resolution
- ✅ curl downloads exact files
- ✅ No node_modules bloat
- ✅ Full control over file locations

### Cons
- ⚠️ Manual URL-to-path mapping
- ⚠️ Need to update import map manually
- ⚠️ Doesn't handle nested dependencies automatically

## Complete Working Example

```bash
#!/bin/bash
# complete-vendor.sh

PROJECT="my-app"

# Setup
mkdir $PROJECT && cd $PROJECT
echo '{"name":"'$PROJECT'","type":"module"}' > package.json

# Generate import map
echo "📦 Generating import map..."
npx jspm link react --out importmap.json

# Create vendor directory
mkdir vendor

# Download files
echo "📥 Downloading files..."
cat importmap.json | jq -r '.imports | to_entries[] | @base64' | while read -r row; do
  _jq() {
    echo $row | base64 -d | jq -r $1
  }

  name=$(_jq '.key')
  url=$(_jq '.value')

  if [[ $url == http* ]]; then
    filename=$(echo "$name" | sed 's/\//-/g').js
    echo "  $name -> vendor/$filename"
    curl -sS -o "vendor/$filename" "$url"
  fi
done

# Update import map
echo "📝 Updating import map..."
cat importmap.json | jq '.imports |= with_entries(
  .value = if .value | startswith("http")
    then "./vendor/" + (.key | gsub("/"; "-")) + ".js"
    else .value
  end
)' > importmap.new.json

mv importmap.new.json importmap.json

echo "✅ Done!"
echo ""
echo "File structure:"
tree -L 2

echo ""
echo "Import map:"
cat importmap.json
```

## Summary

**The Workflow:**
1. `npx jspm link <package> --out importmap.json` → Get CDN URLs
2. `curl -o vendor/file.js <CDN-URL>` → Download to local
3. Edit `importmap.json` → Change URLs to `./vendor/file.js`

**Result:**
- Import map with local paths
- ESM files vendored in `./vendor/`
- No CDN, fully offline

## Quick Reference

| Step | Command | Output |
|------|---------|--------|
| 1. Generate | `npx jspm link react --out importmap.json` | Import map with URLs |
| 2. Create dir | `mkdir vendor` | vendor/ directory |
| 3. Download | `curl -o vendor/react.js "<URL>"` | ESM file in vendor/ |
| 4. Update | Edit importmap.json | Local paths |

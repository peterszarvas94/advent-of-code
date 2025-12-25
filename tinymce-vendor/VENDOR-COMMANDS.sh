#!/bin/bash
# EXACT COMMANDS TO VENDOR ESM FILES WITH JSPM

# ==============================================================================
# METHOD 1: Vendor using esm.sh (RECOMMENDED - Pure ESM)
# ==============================================================================

# Step 1: Create directory structure
mkdir -p tinymce-vendor/vendor
cd tinymce-vendor

# Step 2: Download TinyMCE ESM from esm.sh
curl -o vendor/tinymce.js "https://esm.sh/tinymce@7"

# Step 3: Create import map pointing to local files
cat > importmap.json << 'EOF'
{
  "imports": {
    "tinymce": "./vendor/tinymce.js"
  }
}
EOF

# Done! You now have:
# - vendor/tinymce.js (ESM file)
# - importmap.json (points to local file)


# ==============================================================================
# METHOD 2: Vendor using jsdelivr +esm
# ==============================================================================

# Download from jsdelivr's ESM CDN
curl -o vendor/tinymce.js "https://cdn.jsdelivr.net/npm/tinymce@7/+esm"

# Create import map
cat > importmap.json << 'EOF'
{
  "imports": {
    "tinymce": "./vendor/tinymce.js"
  }
}
EOF


# ==============================================================================
# METHOD 3: Use JSPM Generator API (downloads all dependencies)
# ==============================================================================

npm install -g jspm

# Generate and vendor ALL dependencies locally
jspm link tinymce --provider esm.sh
# Then manually download files from the URLs in importmap.json


# ==============================================================================
# METHOD 4: Vendor with dependencies using import-map-downloader
# ==============================================================================

npm install -g import-map-downloader

# Create initial import map
echo '{"imports":{"tinymce":"https://esm.sh/tinymce@7"}}' > importmap.json

# Download all imports to vendor/ directory
import-map-download importmap.json --output vendor/

# Updates importmap.json to point to local files


# ==============================================================================
# METHOD 5: Manual vendoring with wget (download everything)
# ==============================================================================

mkdir -p vendor

# Download main module
wget -O vendor/tinymce.js https://esm.sh/tinymce@7

# Create import map
cat > importmap.json << 'EOF'
{
  "imports": {
    "tinymce": "./vendor/tinymce.js"
  }
}
EOF


# ==============================================================================
# VERIFICATION
# ==============================================================================

# Check files exist
ls -lh vendor/
cat importmap.json

# Verify it's ESM (should see 'export' keyword)
head -50 vendor/tinymce.js | grep -E "export|import"


# ==============================================================================
# USAGE IN YOUR CODE
# ==============================================================================

# Create test.js
cat > test.js << 'EOF'
import tinymce from 'tinymce';

console.log('TinyMCE loaded:', tinymce);
EOF

# Run with Deno (supports import maps natively)
deno run --import-map=importmap.json test.js

# Or serve with a server that supports import maps
npx http-server -p 8000

#!/bin/bash
# Complete JSPM + curl vendor workflow

# Step 1: Generate import map with JSPM (creates CDN URLs)
echo "Step 1: Generate import map with CDN URLs..."
npx jspm link react --out importmap.json

# Step 2: Create vendor directory
echo "Step 2: Create vendor directory..."
mkdir -p vendor

# Step 3: Extract URLs from import map and download with curl
echo "Step 3: Download files to vendor/..."

# Read import map and extract URLs
# For each URL in the import map, curl it to vendor/

# Example for react:
# From: "react": "https://ga.jspm.io/npm:react@19.1.0/index.js"
# To:   vendor/react.js

curl -o vendor/react.js "https://ga.jspm.io/npm:react@19.1.0/index.js"
curl -o vendor/react-jsx-runtime.js "https://ga.jspm.io/npm:react@19.1.0/jsx-runtime.js"

# Step 4: Update import map to point to local files
echo "Step 4: Update import map to point to local files..."
cat > importmap.json << 'EOF'
{
  "imports": {
    "react": "./vendor/react.js",
    "react/jsx-runtime": "./vendor/react-jsx-runtime.js"
  }
}
EOF

echo "✅ Done! Files vendored to ./vendor/ and import map updated."

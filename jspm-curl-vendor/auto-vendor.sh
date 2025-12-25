#!/bin/bash
# Automated script to vendor JSPM import map URLs

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 JSPM Curl Vendor Automation${NC}"

# Step 1: Generate import map with JSPM
echo -e "\n${GREEN}Step 1: Generate import map...${NC}"
npx jspm link react --out importmap.json

# Step 2: Create vendor directory
echo -e "\n${GREEN}Step 2: Create vendor directory...${NC}"
mkdir -p vendor

# Step 3: Extract and download all URLs from import map
echo -e "\n${GREEN}Step 3: Download all imports to vendor/...${NC}"

# Parse JSON and extract URLs using jq
jq -r '.imports | to_entries[] | "\(.key)|\(.value)"' importmap.json | while IFS='|' read -r name url; do
  if [[ $url == http* ]]; then
    # Generate a safe filename from the import name
    filename=$(echo "$name" | sed 's/\//-/g').js

    echo "  📥 Downloading $name -> vendor/$filename"
    curl -sS -o "vendor/$filename" "$url"

    # Update the import map entry (we'll rebuild the whole map after)
    echo "$name|./vendor/$filename" >> /tmp/vendor-mappings.txt
  fi
done

# Step 4: Rebuild import map with local paths
echo -e "\n${GREEN}Step 4: Update import map to point to local files...${NC}"

# Start building new import map
echo '{' > importmap.json
echo '  "imports": {' >> importmap.json

# Add each mapping
first=true
while IFS='|' read -r name path; do
  if [ "$first" = true ]; then
    first=false
  else
    echo ',' >> importmap.json
  fi
  echo -n "    \"$name\": \"$path\"" >> importmap.json
done < /tmp/vendor-mappings.txt

echo '' >> importmap.json
echo '  }' >> importmap.json
echo '}' >> importmap.json

# Cleanup
rm -f /tmp/vendor-mappings.txt

echo -e "\n${GREEN}✅ Done!${NC}"
echo "  📁 Files vendored to ./vendor/"
echo "  📄 Import map updated to point to local files"
echo ""
cat importmap.json

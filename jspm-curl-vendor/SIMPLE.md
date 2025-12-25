# Simple JSPM + curl Vendor

## The 4 Commands

```bash
# 1. Generate import map (JSPM creates URLs)
npx jspm link react --out importmap.json

# 2. Create vendor directory
mkdir vendor

# 3. curl the URL to vendor/
curl -o vendor/react.js "https://ga.jspm.io/npm:react@19.1.0/index.js"

# 4. Update import map to point to local file
cat > importmap.json << 'EOF'
{
  "imports": {
    "react": "./vendor/react.js"
  }
}
EOF
```

## What Happens

**After Step 1:**
```json
{
  "imports": {
    "react": "https://ga.jspm.io/npm:react@19.1.0/index.js"
  }
}
```

**After Step 3:**
```
vendor/
└── react.js   ← Downloaded ESM file
```

**After Step 4:**
```json
{
  "imports": {
    "react": "./vendor/react.js"
  }
}
```

## Result

- ✅ Import map points to local file
- ✅ ESM JavaScript file in `vendor/`
- ✅ No CDN dependency
- ✅ Works offline

## Use It

```javascript
import React from 'react';  // Uses ./vendor/react.js
```

---

## Full Example

```bash
# Complete workflow
mkdir my-project && cd my-project

# Step 1: Generate import map with CDN URL
echo '{"name":"demo","type":"module"}' > package.json
npx jspm link react --out importmap.json

# importmap.json now contains:
# {
#   "imports": {
#     "react": "https://ga.jspm.io/npm:react@19.1.0/index.js"
#   }
# }

# Step 2: Download the URL to local vendor/
mkdir vendor
curl -o vendor/react.js "https://ga.jspm.io/npm:react@19.1.0/index.js"

# Step 3: Update import map to point to local file
cat > importmap.json << 'EOF'
{
  "imports": {
    "react": "./vendor/react.js"
  }
}
EOF

# Done! You now have:
# - vendor/react.js (the actual ESM file)
# - importmap.json (points to ./vendor/react.js)
```

## File Structure

```
my-project/
├── vendor/
│   └── react.js          ← ESM file (downloaded)
├── importmap.json        ← Points to ./vendor/react.js
└── package.json
```

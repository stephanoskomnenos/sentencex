#!/usr/bin/env bash
set -e

if ! [ -x "$(command -v wasm-pack)" ]; then
  echo "wasm-pack is not installed" >&2
  echo "Install it using:"
  echo "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
  exit 1
fi

if [ -d "pkg" ]; then
  rm -rf pkg
fi

echo "Building for Node.js..."
wasm-pack build -t nodejs -d pkg/nodejs --scope stevenlin

echo "Building for Web..."
wasm-pack build -t web -d pkg/web --scope stevenlin

echo "Building for Bundler..."
wasm-pack build -t bundler -d pkg/bundler --scope stevenlin

cp package.json pkg/
cp README.md pkg/
cp LICENSE pkg/

# Remove duplicate README.md, LICENSE and .gitignore from subdirectories
# (.gitignore with '*' would cause npm to ignore all files in these directories)
rm -f pkg/nodejs/README.md pkg/nodejs/LICENSE pkg/nodejs/.gitignore pkg/nodejs/package.json
rm -f pkg/web/README.md pkg/web/LICENSE pkg/web/.gitignore pkg/web/package.json
rm -f pkg/bundler/README.md pkg/bundler/LICENSE pkg/bundler/.gitignore pkg/bundler/package.json

echo "Build complete!"

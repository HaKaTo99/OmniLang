<#
.SYNOPSIS
    OmniLang Windows Release Build Script
.DESCRIPTION
    Script ini menangani kompilasi lintas platform Standalone Binary (omnilang.exe)
    dan mengemasnya dalam sebuah berkas ZIP beserta dengan dokumentasi Quickstart.
#>

$ErrorActionPreference = "Stop"

$VERSION = "2.2.0"
$TARGET_DIR = "target\release"
$RELEASE_DIR = "omnilang-v$VERSION-windows-x64"

Write-Host "🚀 Membangun Standalone Binary OmniLang v$VERSION untuk Windows..." -ForegroundColor Cyan

# 1. Build The Release Binary
Write-Host "⚙️ Mengeksekusi 'cargo build --release'..." -ForegroundColor Yellow
cargo build --release

# 2. Setup Release Directory
if (Test-Path $RELEASE_DIR) {
    Remove-Item -Recurse -Force $RELEASE_DIR
}
New-Item -ItemType Directory -Force -Path $RELEASE_DIR | Out-Null

# 3. Copy Output Binary & Essential Files
Write-Host "📦 Menyalin bundel biner dan dokumen pelengkap..." -ForegroundColor Yellow
Copy-Item -Path "$TARGET_DIR\omnilang.exe" -Destination "$RELEASE_DIR\" -ErrorAction Stop
Copy-Item -Path "README.md" -Destination "$RELEASE_DIR\" -ErrorAction Stop
Copy-Item -Path "docs\QUICKSTART.md" -Destination "$RELEASE_DIR\" -ErrorAction Stop
Copy-Item -Path "examples" -Destination "$RELEASE_DIR\" -Recurse -ErrorAction Stop

# 4. Create Archive
$ZipFile = "omnilang-v$VERSION-windows-x64.zip"
if (Test-Path $ZipFile) {
    Remove-Item -Force $ZipFile
}
Write-Host "🗜️ Membuat arsip rilis ($ZipFile)..." -ForegroundColor Yellow
Compress-Archive -Path "$RELEASE_DIR" -DestinationPath "$ZipFile"

Write-Host "✅ Berhasil! Rilis standalone siap di: $ZipFile" -ForegroundColor Green

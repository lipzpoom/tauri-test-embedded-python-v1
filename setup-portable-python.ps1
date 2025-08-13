# Download and setup portable Python for Tauri bundle
param(
    [string]$PythonVersion = "3.11.9"
)

$ErrorActionPreference = "Stop"

Write-Host "Setting up portable Python $PythonVersion for Tauri bundle..." -ForegroundColor Green

# Define paths
$tauriDir = "src-tauri"
$pythonDir = "$tauriDir\python"
$pythonBinDir = "$pythonDir\python-dist"
$zipFile = "python-$PythonVersion-embed-amd64.zip"
$downloadUrl = "https://www.python.org/ftp/python/$PythonVersion/$zipFile"

# Ensure we're in the right directory
if (!(Test-Path $tauriDir)) {
    Write-Host "Error: Not in project root. Please run from project root directory." -ForegroundColor Red
    exit 1
}

# Create python distribution directory
if (!(Test-Path $pythonBinDir)) {
    New-Item -ItemType Directory -Path $pythonBinDir -Force
}

# Check if Python is already installed
$pythonExe = Join-Path $pythonBinDir "python.exe"
if (Test-Path $pythonExe) {
    Write-Host "Python already exists in $pythonBinDir" -ForegroundColor Yellow
    $response = Read-Host "Do you want to reinstall? (y/N)"
    if ($response -ne "y" -and $response -ne "Y") {
        Write-Host "Keeping existing Python installation" -ForegroundColor Green
        exit 0
    }
    
    # Remove existing installation
    Remove-Item -Path "$pythonBinDir\*" -Recurse -Force
}

# Download Python embeddable package
Write-Host "Downloading Python from: $downloadUrl" -ForegroundColor Blue
try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $zipFile -UseBasicParsing
    Write-Host "Downloaded successfully" -ForegroundColor Green
} catch {
    Write-Host "Failed to download Python: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# Extract Python to distribution directory
Write-Host "Extracting Python..." -ForegroundColor Blue
try {
    Expand-Archive -Path $zipFile -DestinationPath $pythonBinDir -Force
    Write-Host "Extracted successfully" -ForegroundColor Green
} catch {
    Write-Host "Failed to extract Python: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# Clean up zip file
Remove-Item $zipFile

# Configure Python path file
$pthFile = Join-Path $pythonBinDir "python311._pth"
if (Test-Path $pthFile) {
    $content = Get-Content $pthFile
    $newContent = $content | ForEach-Object {
        if ($_ -match "^#import site") {
            "import site"
        } else {
            $_
        }
    }
    if ($newContent -notcontains "import site") {
        $newContent += "import site"
    }
    $newContent | Set-Content $pthFile
}

# Copy Python executable to main python directory for easy access
Copy-Item "$pythonBinDir\python.exe" "$pythonDir\python.exe" -Force

# Test Python installation
Write-Host "Testing Python installation..." -ForegroundColor Blue
try {
    $version = & "$pythonDir\python.exe" --version 2>&1
    Write-Host "Python installed successfully: $version" -ForegroundColor Green
} catch {
    Write-Host "Python test failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# Download get-pip.py
$getPipUrl = "https://bootstrap.pypa.io/get-pip.py"
$getPipFile = Join-Path $pythonBinDir "get-pip.py"
Write-Host "Downloading get-pip.py..." -ForegroundColor Blue
try {
    Invoke-WebRequest -Uri $getPipUrl -OutFile $getPipFile -UseBasicParsing
} catch {
    Write-Host "Failed to download get-pip.py: $($_.Exception.Message)" -ForegroundColor Yellow
}

# Install pip and required packages
if (Test-Path $getPipFile) {
    Write-Host "Installing pip..." -ForegroundColor Blue
    try {
        & "$pythonDir\python.exe" $getPipFile --no-warn-script-location
        Write-Host "Pip installed successfully" -ForegroundColor Green
        
        # Install required packages
        Write-Host "Installing required packages..." -ForegroundColor Blue
        & "$pythonDir\python.exe" -m pip install --no-warn-script-location requests
        Write-Host "Packages installed successfully" -ForegroundColor Green
    } catch {
        Write-Host "Pip/packages installation failed: $($_.Exception.Message)" -ForegroundColor Yellow
    }
}

Write-Host "Portable Python setup completed!" -ForegroundColor Green
Write-Host "Python executable: $pythonDir\python.exe"
Write-Host "Distribution directory: $pythonBinDir"
Write-Host ""
Write-Host "Files will be bundled with Tauri when you run 'npm run tauri build'" -ForegroundColor Cyan

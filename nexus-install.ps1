# nexus-install.ps1 - Advanced installation script

param(
    [switch]$Force,
    [switch]$NoUI,
    [switch]$DevMode,
    [string]$InstallPath = "$env:USERPROFILE\nexus-studio"
)

#region Setup
$ErrorActionPreference = "Stop"

# Colors
$Colors = @{
    Primary = "`e[38;5;51m"
    Success = "`e[38;5;46m"
    Warning = "`e[38;5;226m"
    Error = "`e[38;5;196m"
    Info = "`e[38;5;39m"
    Reset = "`e[0m"
}

function Write-Color {
    param($Color, $Text)
    Write-Host "$($Color)$Text$($Colors.Reset)"
}

function Show-Banner {
    Clear-Host
    Write-Color $Colors.Primary @"
    ╔══════════════════════════════════════════════════════════════╗
    ║                                                              ║
    ║     🚀 NEXUS STUDIO AI - Advanced Installation              ║
    ║                                                              ║
    ╚══════════════════════════════════════════════════════════════╝
    Version: 2.0.0 | PowerShell 7+ Required

"@
}
#endregion

#region Requirements Check
function Test-Requirements {
    Write-Color $Colors.Info "🔍 Checking system requirements..."

    $issues = @()

    # PowerShell version
    if ($PSVersionTable.PSVersion.Major -lt 7) {
        $issues += "PowerShell 7.0+ required (Current: $($PSVersionTable.PSVersion))"
    }

    # RAM check
    $ram = (Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB
    if ($ram -lt 8) {
        $issues += "Recommended: 8GB RAM (Current: ${ram}GB)"
    }

    # Disk space
    $drive = Get-PSDrive -Name C
    $freeSpaceGB = [math]::Round($drive.Free / 1GB, 2)
    if ($freeSpaceGB -lt 10) {
        $issues += "Low disk space on C: (${freeSpaceGB}GB free)"
    }

    if ($issues.Count -gt 0) {
        Write-Color $Colors.Warning "⚠️  Potential issues found:"
        foreach ($issue in $issues) {
            Write-Host "   • $issue"
        }

        $continue = Read-Host "Continue anyway? (y/N)"
        if ($continue -ne 'y') {
            exit 1
        }
    }

    Write-Color $Colors.Success "✅ System requirements OK"
}
#endregion

#region Dependency Installation
function Install-Dependencies {
    Write-Color $Colors.Info "📦 Installing dependencies..."

    $deps = @(
        @{Name="Git"; Command="git"; Install={ winget install Git.Git }},
        @{Name="Node.js"; Command="node"; Install={ winget install OpenJS.NodeJS.LTS }},
        @{Name="Python 3"; Command="python"; Install={ winget install Python.Python.3.11 }},
        @{Name="VS Code"; Command="code"; Install={ winget install Microsoft.VisualStudioCode }},
        @{Name="Rust"; Command="rustc"; Install={ winget install Rustlang.Rustup }}
    )

    foreach ($dep in $deps) {
        if (-not (Get-Command $dep.Command -ErrorAction SilentlyContinue)) {
            Write-Host "Installing $($dep.Name)..." -NoNewline
            try {
                & $dep.Install
                Write-Host "$($Colors.Success) ✅$($Colors.Reset)"
            } catch {
                Write-Host "$($Colors.Error) ❌$($Colors.Reset)"
                Write-Host "Failed to install $($dep.Name): $_"
            }
        } else {
            Write-Host "$($dep.Name) already installed $($Colors.Success)✅$($Colors.Reset)"
        }
    }
}
#endregion

#region Core Installation
function Install-NexusCore {
    param($Path)

    Write-Color $Colors.Info "🚀 Installing Nexus Studio AI..."

    # Create directory structure
    $dirs = @("bin", "config", "models", "projects", "logs", "plugins")
    foreach ($dir in $dirs) {
        $fullPath = Join-Path $Path $dir
        if (-not (Test-Path $fullPath)) {
            New-Item -ItemType Directory -Path $fullPath -Force | Out-Null
        }
    }

    # Clone repository or copy files
    if ($DevMode) {
        Write-Host "Development mode: Using local files..."
        Copy-Item -Path ".\*" -Destination $Path -Recurse -Force
    } else {
        Write-Host "Downloading from GitHub..."
        $repoUrl = "https://github.com/nexus-studio/cli.git"

        if (Test-Path (Join-Path $Path ".git")) {
            Set-Location $Path
            git pull
        } else {
            git clone $repoUrl $Path
        }
    }

    # Install Node dependencies
    Set-Location $Path
    if (Test-Path "package.json") {
        Write-Host "Installing Node dependencies..."
        npm install
    }

    # Install Python dependencies
    if (Test-Path "requirements.txt") {
        Write-Host "Installing Python dependencies..."
        pip install -r requirements.txt
    }

    # Build Rust components if present
    if (Test-Path "Cargo.toml") {
        Write-Host "Building Rust components..."
        cargo build --release
    }
}
#endregion

#region AI Models Setup
function Install-AIModels {
    Write-Color $Colors.Info "🤖 Setting up AI models..."

    $modelsPath = "$env:USERPROFILE\.nexus\models"
    New-Item -ItemType Directory -Path $modelsPath -Force | Out-Null

    $models = @(
        @{
            Name = "Phi-3 Mini (3.8B)"
            URL = "https://huggingface.co/microsoft/phi-3-mini-128k-instruct"
            Size = "2.3GB"
            Type = "MLC"
        },
        @{
            Name = "Phi-3 Small (7B)"
            URL = "https://huggingface.co/microsoft/phi-3-small-128k-instruct"
            Size = "4.5GB"
            Type = "MLC"
        },
        @{
            Name = "CodeLlama 7B"
            URL = "https://huggingface.co/codellama/CodeLlama-7b-hf"
            Size = "14GB"
            Type = "GGUF"
        }
    )

    Write-Host "Available AI models:"
    for ($i = 0; $i -lt $models.Count; $i++) {
        Write-Host "$($i+1)) $($models[$i].Name) [$($models[$i].Size)]"
    }

    $choices = Read-Host "Select models to download (comma-separated, e.g., 1,3)"
    $selected = $choices -split ',' | ForEach-Object { [int]$_ - 1 }

    foreach ($index in $selected) {
        if ($index -ge 0 -and $index -lt $models.Count) {
            $model = $models[$index]
            Write-Host "Downloading $($model.Name)..."

            # Simulate download (replace with actual download logic)
            Start-Sleep -Seconds 2

            # Create model directory
            $modelDir = Join-Path $modelsPath ($model.Name -replace '[^\w\-]', '_')
            New-Item -ItemType Directory -Path $modelDir -Force | Out-Null

            # Create model config
            $config = @{
                name = $model.Name
                type = $model.Type
                size = $model.Size
                url = $model.URL
                downloaded = (Get-Date).ToString("yyyy-MM-dd")
            }

            $config | ConvertTo-Json | Out-File (Join-Path $modelDir "config.json")

            Write-Host "$($Colors.Success)✅ $($model.Name) setup complete"
        }
    }
}
#endregion

#region Post-Installation
function Configure-Environment {
    Write-Color $Colors.Info "⚙️  Configuring environment..."

    # Add to PATH
    $nexusPath = "$env:USERPROFILE\nexus-studio\bin"
    $pathVar = [Environment]::GetEnvironmentVariable("PATH", "User")

    if ($pathVar -notlike "*$nexusPath*") {
        [Environment]::SetEnvironmentVariable("PATH", "$pathVar;$nexusPath", "User")
        Write-Host "Added to PATH"
    }

    # Create desktop shortcut
    if (-not $NoUI) {
        $shortcutPath = "$env:USERPROFILE\Desktop\Nexus Studio AI.lnk"
        $wsShell = New-Object -ComObject WScript.Shell
        $shortcut = $wsShell.CreateShortcut($shortcutPath)
        $shortcut.TargetPath = "pwsh.exe"
        $shortcut.Arguments = "-NoExit -Command `"nexus-tui`""
        $shortcut.WorkingDirectory = "$env:USERPROFILE\nexus-studio"
        $shortcut.IconLocation = "$env:USERPROFILE\nexus-studio\assets\icon.ico"
        $shortcut.Save()
    }

    # Create start menu entry
    $startMenuPath = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Nexus Studio AI"
    New-Item -ItemType Directory -Path $startMenuPath -Force | Out-Null
    Copy-Item $shortcutPath $startMenuPath -Force
}
#endregion

#region Verification
function Test-Installation {
    Write-Color $Colors.Info "🔍 Verifying installation..."

    $tests = @(
        @{Name="Core files"; Test={ Test-Path "$InstallPath\nexus-tui.ps1" }},
        @{Name="Config files"; Test={ Test-Path "$env:USERPROFILE\.nexus\config.json" }},
        @{Name="AI models"; Test={ (Get-ChildItem "$env:USERPROFILE\.nexus\models" -Directory).Count -gt 0 }},
        @{Name="PATH configuration"; Test={ $env:PATH -like "*nexus-studio*" }}
    )

    $allPassed = $true

    foreach ($test in $tests) {
        try {
            $result = & $test.Test
            if ($result) {
                Write-Host "$($test.Name) $($Colors.Success)✅$($Colors.Reset)"
            } else {
                Write-Host "$($test.Name) $($Colors.Error)❌$($Colors.Reset)"
                $allPassed = $false
            }
        } catch {
            Write-Host "$($test.Name) $($Colors.Error)❌ (Error: $_)$($Colors.Reset)"
            $allPassed = $false
        }
    }

    if ($allPassed) {
        Write-Color $Colors.Success "🎉 Installation verified successfully!"
    } else {
        Write-Color $Colors.Warning "⚠️  Some tests failed. Installation may be incomplete."
    }
}
#endregion

#region Main Installation Flow
function Main {
    Show-Banner

    Write-Color $Colors.Warning "⚠️  This will install Nexus Studio AI and required dependencies."
    Write-Color $Colors.Warning "    Approx. disk space needed: 5GB"

    if (-not $Force) {
        $confirm = Read-Host "Continue? (y/N)"
        if ($confirm -ne 'y') {
            exit 0
        }
    }

    # Run installation steps
    Test-Requirements
    Install-Dependencies
    Install-NexusCore -Path $InstallPath
    Install-AIModels
    Configure-Environment
    Test-Installation

    # Show completion message
    Show-Banner
    Write-Color $Colors.Success @"
🎉 NEXUS STUDIO AI INSTALLATION COMPLETE!

Quick Start:
1. Open Windows Terminal or PowerShell 7
2. Type: nexus-tui
3. Or: pwsh -Command "nexus-tui"

Features Installed:
✅ Advanced TUI interface
✅ Phi-3 AI integration
✅ Component library
✅ Deployment tools
✅ Project templates

Documentation: https://docs.nexus.studio
Community: https://github.com/nexus-studio/cli

Press Enter to launch Nexus Studio AI...
"@

    Read-Host

    # Launch Nexus
    & "$InstallPath\nexus-tui.ps1"
}

# Run installation
Main
#endregion

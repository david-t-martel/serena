#!/usr/bin/env pwsh
# File watcher for automatic git staging of documentation files
# Run this script in the background to auto-stage markdown files

param(
    [switch]$Verbose
)

$watchPath = "T:/projects/serena-source"
$filter = "*.md"

Write-Host "🔍 Starting documentation file watcher..." -ForegroundColor Cyan
Write-Host "📁 Watching: $watchPath" -ForegroundColor Gray
Write-Host "📄 Filter: $filter" -ForegroundColor Gray
Write-Host "Press Ctrl+C to stop" -ForegroundColor Yellow
Write-Host ""

$watcher = New-Object System.IO.FileSystemWatcher
$watcher.Path = $watchPath
$watcher.Filter = $filter
$watcher.IncludeSubdirectories = $false
$watcher.EnableRaisingEvents = $true

# Debounce mechanism to avoid multiple triggers
$script:lastEventTime = @{}
$debounceSeconds = 2

$action = {
    $path = $Event.SourceEventArgs.FullPath
    $changeType = $Event.SourceEventArgs.ChangeType
    $timeStamp = $Event.TimeGenerated
    
    # Debounce: ignore if same file was modified within last 2 seconds
    $fileName = Split-Path $path -Leaf
    $now = Get-Date
    if ($script:lastEventTime.ContainsKey($fileName)) {
        $timeDiff = ($now - $script:lastEventTime[$fileName]).TotalSeconds
        if ($timeDiff -lt $debounceSeconds) {
            return
        }
    }
    $script:lastEventTime[$fileName] = $now
    
    # Skip temp/backup files
    if ($fileName -match '~$|\.tmp$|\.bak$') {
        return
    }
    
    Write-Host "[$timeStamp] $changeType : $fileName" -ForegroundColor Green
    
    # Auto-stage the file
    Push-Location $watchPath
    try {
        $gitStatus = git status --porcelain $fileName 2>&1
        if ($gitStatus -match '^\s*M\s+' -or $gitStatus -match '^\?\?') {
            git add $fileName
            Write-Host "  ✓ Staged: $fileName" -ForegroundColor Cyan
        }
    } catch {
        Write-Host "  ✗ Error staging: $_" -ForegroundColor Red
    }
    Pop-Location
}

# Register event handlers
$handlers = @()
$handlers += Register-ObjectEvent -InputObject $watcher -EventName "Changed" -Action $action
$handlers += Register-ObjectEvent -InputObject $watcher -EventName "Created" -Action $action
$handlers += Register-ObjectEvent -InputObject $watcher -EventName "Renamed" -Action $action

try {
    while ($true) {
        Start-Sleep -Seconds 1
    }
} finally {
    # Cleanup on exit
    $handlers | Unregister-Event
    $watcher.Dispose()
    Write-Host "`n👋 File watcher stopped" -ForegroundColor Yellow
}

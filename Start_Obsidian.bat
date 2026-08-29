@echo off
title Project Obsidian - Privacy, Gaming & AI Workstation
cd /d "%~dp0"

:: Check for Administrator Privileges
net session >nul 2>&1
if %errorlevel% == 0 (
    if exist "obsidian.exe" (
        "obsidian.exe"
    ) else if exist "target\release\obsidian.exe" (
        "target\release\obsidian.exe"
    ) else (
        powershell -NoProfile -ExecutionPolicy Bypass -File "Obsidian.ps1"
    )
) else (
    echo ==============================================================================
    echo   Project Obsidian requires Administrator privileges to configure Windows.
    echo   Requesting elevation...
    echo ==============================================================================
    powershell -NoProfile -Command "Start-Process cmd.exe -ArgumentList '/c \"\"%~f0\"\"' -Verb RunAs"
)

@echo off
setlocal
rem Images Scroller - default image viewer registration helper (Windows 10/11).
rem
rem Run this script from INSIDE the extracted Images Scroller folder, next to
rem img-scroller.exe. It registers Images Scroller for YOUR Windows user only
rem (HKCU registry keys; no administrator rights) and then opens the Windows
rem Settings "Default apps" page, where you pick Images Scroller for the file
rem types you want. Windows never lets a script set a default silently - the
rem final per-type choice is always yours in Settings.
rem
rem Usage:
rem   register-default-viewer.cmd                 register (or re-register)
rem   register-default-viewer.cmd --unregister    remove the registration
rem Running it again is safe: it writes the same registration values again.

set REGISTRY_PROGID=ImagesScroller.Image
set REGISTRY_APP_KEY=HKCU\Software\Images Scroller
set REGISTRY_APP_NAME=Images Scroller

if "%~1"=="" goto register
if "%~1"=="--unregister" goto unregister
echo Unknown argument: %~1
echo Usage: register-default-viewer.cmd [--unregister]
exit /b 1

:register
set APP_EXE=%~dp0img-scroller.exe
if not exist "%APP_EXE%" goto missing_exe
rem The application's own ProgID: how Windows describes and launches the app.
reg add "HKCU\Software\Classes\%REGISTRY_PROGID%" /ve /d "Images Scroller Image" /f >nul
reg add "HKCU\Software\Classes\%REGISTRY_PROGID%" /v "FriendlyTypeName" /t REG_SZ /d "Images Scroller Image" /f >nul
reg add "HKCU\Software\Classes\%REGISTRY_PROGID%\DefaultIcon" /ve /d "\"%APP_EXE%\",0" /f >nul
reg add "HKCU\Software\Classes\%REGISTRY_PROGID%\shell\open\command" /ve /d "\"%APP_EXE%\" \"%%1\"" /f >nul
rem Per-extension "Open with" entries (right-click an image, "Open with") and
rem Capabilities file associations (the Settings "Default apps" page).
for %%E in (avif bmp dib gif heic heif ico jfif jpe jpeg jpg jxr png tif tiff webp wdp) do (
  reg add "HKCU\Software\Classes\.%%E\OpenWithProgids" /v "%REGISTRY_PROGID%" /t REG_SZ /d "" /f >nul
  reg add "%REGISTRY_APP_KEY%\Capabilities\FileAssociations" /v ".%%E" /t REG_SZ /d "%REGISTRY_PROGID%" /f >nul
)
reg add "%REGISTRY_APP_KEY%\Capabilities" /v "ApplicationName" /t REG_SZ /d "%REGISTRY_APP_NAME%" /f >nul
reg add "%REGISTRY_APP_KEY%\Capabilities" /v "ApplicationIcon" /t REG_SZ /d "\"%APP_EXE%\",0" /f >nul
reg add "%REGISTRY_APP_KEY%\Capabilities" /v "ApplicationDescription" /t REG_SZ /d "Classic gallery-style image viewer: open one image and scroll through the sibling images and sibling folders with the arrow keys." /f >nul
reg add "HKCU\Software\RegisteredApplications" /v "%REGISTRY_APP_NAME%" /t REG_SZ /d "Software\Images Scroller\Capabilities" /f >nul
echo Images Scroller is registered for this Windows user.
echo Windows opened the "Default apps" Settings page: pick Images Scroller
echo there for the file types you want it to open.
start "" "ms-settings:defaultapps"
exit /b 0

:unregister
reg delete "HKCU\Software\Classes\%REGISTRY_PROGID%" /f >nul 2>&1
for %%E in (avif bmp dib gif heic heif ico jfif jpe jpeg jpg jxr png tif tiff webp wdp) do reg delete "HKCU\Software\Classes\.%%E\OpenWithProgids" /v "%REGISTRY_PROGID%" /f >nul 2>&1
reg delete "%REGISTRY_APP_KEY%" /f >nul 2>&1
reg delete "HKCU\Software\RegisteredApplications" /v "%REGISTRY_APP_NAME%" /f >nul 2>&1
echo Images Scroller registration removed for this Windows user.
echo If it was set as the default for a file type, pick a different
echo default for that type in Windows Settings (Apps, Default apps).
exit /b 0

:missing_exe
echo img-scroller.exe was not found next to this script.
echo Run this script from inside the extracted Images Scroller folder.
exit /b 1

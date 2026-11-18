# Changelog

All notable changes to **Images Scroller** are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project uses Semantic Versioning (SemVer).

## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [1.1.0] - 2026-11-18

### Added

- Installs Images Scroller with an executable Windows wizard installer (ImagesScroller-1.1.0-windows-installer.exe, published in the GitHub Releases section): the wizard asks for the destination folder — defaulting to your per-user Programs folder under `%LOCALAPPDATA%` — installs the executable, the config file, and the license, and creates a Start Menu shortcut, all without administrator rights because everything installs for your Windows account only.
- Lets you choose which image file types to offer Images Scroller for on a dedicated file-association wizard page — one checkbox for each of the 17 supported types, none preselected — registering only the types you pick for your Windows account (the "Open with" lists and the Windows Settings "Default apps" visibility, which opens automatically after the install when at least one type was selected); Windows itself always makes the final per-type default choice, and the installer never writes it for you.
- Ships an uninstaller (reachable from Windows' "Installed apps" / Add-or-remove-programs list) that removes the installed files, the Start Menu shortcut, and exactly the associations the install created.
- Shows the path of the currently visible image in the window title, updated on every navigation: the full path when it fits, otherwise the file name with its extension (its last characters only when the name itself is too long) grown by the containing folder, then the drive — with `...` between the drive and the visible folder chain — and more parent folders while space allows; while the window is not maximized the title shows only the file name, and while fullscreen is on a popup at the top center shows the visible image's name for 3 seconds on every image change.
- Publishes the releases in the GitHub Releases section: this 1.1.0 release carries the wizard installer for 64-bit Windows plus the portable zip packages for 64-bit and 32-bit Windows.

### Changed

- Keeps the portable zip packages alongside the wizard installer: each zip still contains exactly the four flat files (the executable, the config file, the register-default-viewer.cmd helper, and DEFAULT-VIEWER-GUIDE.txt), and the 32-bit zip remains the only distribution for 32-bit Windows (the installer embeds the 64-bit executable only).

## [1.0.0] - 2026-11-15

### Added

- Opens any image whose format is registered as a codec in the Windows system (for example jpg, png, bmp, gif, webp, and heic when a system codec is installed) from Explorer, a file manager, or the in-app open dialog, with the window opening maximized at every launch and no console window in the release build, while formats without a system codec show the unsupported-file message.
- Moves to the previous or next sibling image in the current folder with the left and right arrow keys or the on-screen pill buttons, never leaving that folder, in the same name order a Windows file manager shows — including non-ASCII and Japanese names, digit-aware and case-insensitive — and stays put at the first or last image without wrapping.
- Moves to the previous or next sibling folder under the same parent with the up and down arrow keys or the on-screen pill buttons — no grandparent jumps, no recursion, and no wrapping — reopening a folder's last-viewed image when it was already visited this session (remembered only in memory and lost on close) or otherwise always its first image in the system name order, and entering an image-less folder in an "Empty folder" browsing state instead of skipping it.
- Toggles fullscreen with F11 or the on-screen fullscreen button, hiding the window header, borders, and every UI control so only the image shows while the arrow keys keep navigating, and exiting with F11 or ESC.
- Zooms in and out with the I and O keys (plus strict Shift aliases) or the on-screen [+] and [−] buttons, each step multiplying or dividing by 1.25 between symmetric bounds of up to ×8 of the whole-image fit and down to ÷8 of it, clamping silently at the bounds, returning to whole-image fit with the 0 key or the restore button, and setting a one-shot fit-width (W) or fit-height (H) size that zoom continues from rather than a persistent mode.
- Pans a zoomed image that is wider or taller than the window with the arrow keys (about 1/8 of the window per press) and navigates to the next image or folder only when panning reaches the image's edge, where a distinct press navigates because holding the key never does, while arrows on an axis that fits entirely navigate directly.
- Opens, from an on-screen pill button or the Q key, a menu of the programs the Windows system has registered for the current file's type — built from system registration data at runtime with no internal program database — where selecting a program opens that file with it, every other app function is off while the menu is open and ESC or Q again closes it, and a failed launch shows the app's single open-with failure message.
- Closes the application while fullscreen is off only after a second distinct ESC press: the first press shows a centered "Press ESC again to close" popup for about 2 seconds that dismisses itself, a held ESC never counts, an expired popup re-arms on the next press, and while fullscreen is on ESC exits fullscreen only.
- Ships as a portable zip package with no installer, one zip per architecture — ImagesScroller-1.0.0-windows-x64.zip for 64-bit Windows and ImagesScroller-1.0.0-windows-x86.zip for 32-bit Windows — that you extract with the standard Windows extract-all flow into any folder you choose, which becomes the install folder.
- Contains exactly four files at the zip root: the application executable img-scroller.exe, the config file img-scroller.toml, the default-viewer registration helper register-default-viewer.cmd, and the guidance DEFAULT-VIEWER-GUIDE.txt.
- Ships the application config file img-scroller.toml next to the executable even though the current version does not read it, since no settings exist in this release and there is nothing to configure.
- Includes the register-default-viewer.cmd helper and the DEFAULT-VIEWER-GUIDE.txt guidance: running the helper registers Images Scroller for your Windows account only without administrator rights, adds it to the "Open with" lists, makes it visible in the Windows Settings "Default apps" page, and opens that page for you to choose the final default per type, covering the 17 image file types the app supports, safe to re-run, and with register-default-viewer.cmd --unregister removing exactly the entries it created.

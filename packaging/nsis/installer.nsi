; Images Scroller Windows wizard installer (NSIS 3, MUI2 pages + nsDialogs association page).
; Compiled container-side by scripts/build-installer.sh, which injects every define used below.
; Artifact: dist/ImagesScroller-<version>-windows-installer.exe.
; Page order: Welcome, destination Directory, association selection, install, finish.
; Registry scope: per-user HKCU only (the Wave-7 helper surface); UserChoice is NEVER written.

OutFile "${INSTALLER_OUTFILE}"
Name "Images Scroller ${PRODUCT_VERSION}"
InstallDir "$LOCALAPPDATA\Programs\Images Scroller"
InstallDirRegKey HKCU "Software\Images Scroller" "InstallDir"
RequestExecutionLevel user
VIProductVersion "${PRODUCT_VERSION_4}"
VIAddVersionKey "FileDescription" "Images Scroller Installer"
VIAddVersionKey "ProductName" "Images Scroller"
VIAddVersionKey "FileVersion" "${PRODUCT_VERSION_4}"
VIAddVersionKey "ProductVersion" "${PRODUCT_VERSION}"
!include MUI2.nsh
!include nsDialogs.nsh
!include LogicLib.nsh

!define PRODUCT_NAME "Images Scroller"
!define PROGID "ImagesScroller.Image"
!define APP_KEY "Software\Images Scroller"
!define REG_APP_NAME "Images Scroller"
!define UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\Images Scroller"
!define EXTENSION_LIST "avif bmp dib gif heic heif ico jfif jpe jpeg jpg jxr png tif tiff webp wdp"

!macro ASSOC_DECLARE ext
  Var CheckboxHwnd${ext}
  Var CheckboxSelected${ext}
!macroend

!macro ASSOC_CREATE ext col row
  ${NSD_CreateCheckBox} ${col}u ${row}u 42u 12u ".${ext}"
  Pop $CheckboxHwnd${ext}
!macroend

!macro ASSOC_READ ext
  ${NSD_GetState} $CheckboxHwnd${ext} $0
  ${If} $0 = ${BST_CHECKED}
    StrCpy $CheckboxSelected${ext} "1"
    StrCpy $InstallHadSelections "1"
  ${Else}
    StrCpy $CheckboxSelected${ext} "0"
  ${EndIf}
!macroend

!macro ASSOC_CLEAN ext
  DeleteRegValue HKCU "Software\Classes\.${ext}\OpenWithProgids" "${PROGID}"
  DeleteRegValue HKCU "${APP_KEY}\Capabilities\FileAssociations" ".${ext}"
!macroend

!macro ASSOC_WRITE ext
  ${If} $CheckboxSelected${ext} = "1"
    WriteRegStr HKCU "Software\Classes\.${ext}\OpenWithProgids" "${PROGID}" ""
    WriteRegStr HKCU "${APP_KEY}\Capabilities\FileAssociations" ".${ext}" "${PROGID}"
  ${EndIf}
!macroend

!macro WRITE_CAPABILITIES
  WriteRegStr HKCU "${APP_KEY}\Capabilities" "ApplicationName" "${REG_APP_NAME}"
  WriteRegStr HKCU "${APP_KEY}\Capabilities" "ApplicationIcon" '"$INSTDIR\img-scroller.exe",0'
  WriteRegStr HKCU "${APP_KEY}\Capabilities" "ApplicationDescription" "Classic gallery-style image viewer: open one image and scroll through the sibling images and sibling folders with the arrow keys."
  WriteRegStr HKCU "Software\RegisteredApplications" "${REG_APP_NAME}" "${APP_KEY}\Capabilities"
!macroend

Var Dialog
Var InstallHadSelections

!insertmacro ASSOC_DECLARE avif
!insertmacro ASSOC_DECLARE bmp
!insertmacro ASSOC_DECLARE dib
!insertmacro ASSOC_DECLARE gif
!insertmacro ASSOC_DECLARE heic
!insertmacro ASSOC_DECLARE heif
!insertmacro ASSOC_DECLARE ico
!insertmacro ASSOC_DECLARE jfif
!insertmacro ASSOC_DECLARE jpe
!insertmacro ASSOC_DECLARE jpeg
!insertmacro ASSOC_DECLARE jpg
!insertmacro ASSOC_DECLARE jxr
!insertmacro ASSOC_DECLARE png
!insertmacro ASSOC_DECLARE tif
!insertmacro ASSOC_DECLARE tiff
!insertmacro ASSOC_DECLARE webp
!insertmacro ASSOC_DECLARE wdp

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
Page custom ShowAssociationPage LeaveAssociationPage
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_LANGUAGE English

Function .onInit
  SetShellVarContext current
FunctionEnd

Function ShowAssociationPage
  !insertmacro MUI_HEADER_TEXT "File associations" "Choose which image file types Images Scroller will be offered for."
  nsDialogs::Create 1018
  Pop $Dialog
  ${NSD_CreateLabel} 0u 0u 250u 16u "Select the image file types to offer Images Scroller for. You confirm the actual defaults in Windows Settings, which opens after the install if any type is selected."
  Pop $0
  !insertmacro ASSOC_CREATE avif 0 26
  !insertmacro ASSOC_CREATE bmp 0 39
  !insertmacro ASSOC_CREATE dib 0 52
  !insertmacro ASSOC_CREATE gif 0 65
  !insertmacro ASSOC_CREATE heic 0 78
  !insertmacro ASSOC_CREATE heif 0 91
  !insertmacro ASSOC_CREATE ico 50 26
  !insertmacro ASSOC_CREATE jfif 50 39
  !insertmacro ASSOC_CREATE jpe 50 52
  !insertmacro ASSOC_CREATE jpeg 50 65
  !insertmacro ASSOC_CREATE jpg 50 78
  !insertmacro ASSOC_CREATE jxr 50 91
  !insertmacro ASSOC_CREATE png 100 26
  !insertmacro ASSOC_CREATE tif 100 39
  !insertmacro ASSOC_CREATE tiff 100 52
  !insertmacro ASSOC_CREATE webp 100 65
  !insertmacro ASSOC_CREATE wdp 100 78
  nsDialogs::Show
FunctionEnd

Function LeaveAssociationPage
  StrCpy $InstallHadSelections "0"
  !insertmacro ASSOC_READ avif
  !insertmacro ASSOC_READ bmp
  !insertmacro ASSOC_READ dib
  !insertmacro ASSOC_READ gif
  !insertmacro ASSOC_READ heic
  !insertmacro ASSOC_READ heif
  !insertmacro ASSOC_READ ico
  !insertmacro ASSOC_READ jfif
  !insertmacro ASSOC_READ jpe
  !insertmacro ASSOC_READ jpeg
  !insertmacro ASSOC_READ jpg
  !insertmacro ASSOC_READ jxr
  !insertmacro ASSOC_READ png
  !insertmacro ASSOC_READ tif
  !insertmacro ASSOC_READ tiff
  !insertmacro ASSOC_READ webp
  !insertmacro ASSOC_READ wdp
FunctionEnd

Function .onInstSuccess
  ${If} $InstallHadSelections = "1"
    ExecShell "" "ms-settings:defaultapps"
  ${EndIf}
FunctionEnd

Section "Install" SEC_INSTALL
  SetOutPath "$INSTDIR"
  File /oname=img-scroller.exe "${EXE_PAYLOAD}"
  File /oname=img-scroller.toml "${CONFIG_PAYLOAD}"
  File /oname=LICENSE "${LICENSE_PAYLOAD}"
  CreateShortcut "$SMPROGRAMS\Images Scroller.lnk" "$INSTDIR\img-scroller.exe"
  WriteRegStr HKCU "${APP_KEY}" "InstallDir" "$INSTDIR"

  ; Reinstall convergence: drop every previously-created per-type entry first
  !insertmacro ASSOC_CLEAN avif
  !insertmacro ASSOC_CLEAN bmp
  !insertmacro ASSOC_CLEAN dib
  !insertmacro ASSOC_CLEAN gif
  !insertmacro ASSOC_CLEAN heic
  !insertmacro ASSOC_CLEAN heif
  !insertmacro ASSOC_CLEAN ico
  !insertmacro ASSOC_CLEAN jfif
  !insertmacro ASSOC_CLEAN jpe
  !insertmacro ASSOC_CLEAN jpeg
  !insertmacro ASSOC_CLEAN jpg
  !insertmacro ASSOC_CLEAN jxr
  !insertmacro ASSOC_CLEAN png
  !insertmacro ASSOC_CLEAN tif
  !insertmacro ASSOC_CLEAN tiff
  !insertmacro ASSOC_CLEAN webp
  !insertmacro ASSOC_CLEAN wdp

  WriteRegStr HKCU "Software\Classes\${PROGID}" "" "Images Scroller Image"
  WriteRegStr HKCU "Software\Classes\${PROGID}" "FriendlyTypeName" "Images Scroller Image"
  WriteRegStr HKCU "Software\Classes\${PROGID}\DefaultIcon" "" '"$INSTDIR\img-scroller.exe",0'
  WriteRegStr HKCU "Software\Classes\${PROGID}\shell\open\command" "" '"$INSTDIR\img-scroller.exe" "%1"'

  !insertmacro ASSOC_WRITE avif
  !insertmacro ASSOC_WRITE bmp
  !insertmacro ASSOC_WRITE dib
  !insertmacro ASSOC_WRITE gif
  !insertmacro ASSOC_WRITE heic
  !insertmacro ASSOC_WRITE heif
  !insertmacro ASSOC_WRITE ico
  !insertmacro ASSOC_WRITE jfif
  !insertmacro ASSOC_WRITE jpe
  !insertmacro ASSOC_WRITE jpeg
  !insertmacro ASSOC_WRITE jpg
  !insertmacro ASSOC_WRITE jxr
  !insertmacro ASSOC_WRITE png
  !insertmacro ASSOC_WRITE tif
  !insertmacro ASSOC_WRITE tiff
  !insertmacro ASSOC_WRITE webp
  !insertmacro ASSOC_WRITE wdp

  !insertmacro WRITE_CAPABILITIES
  System::Call "shell32::SHChangeNotify(i 0x08000000, i 0, i 0, i 0)"
  WriteUninstaller "$INSTDIR\uninstall.exe"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayName" "${PRODUCT_NAME}"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayIcon" '"$INSTDIR\img-scroller.exe",0'
  WriteRegStr HKCU "${UNINST_KEY}" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegDWORD HKCU "${UNINST_KEY}" "NoModify" 1
  WriteRegDWORD HKCU "${UNINST_KEY}" "NoRepair" 1
SectionEnd

Section "un.Uninstall" SEC_UNINSTALL
  Delete "$INSTDIR\img-scroller.exe"
  Delete "$INSTDIR\img-scroller.toml"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"
  Delete "$SMPROGRAMS\Images Scroller.lnk"
  !insertmacro ASSOC_CLEAN avif
  !insertmacro ASSOC_CLEAN bmp
  !insertmacro ASSOC_CLEAN dib
  !insertmacro ASSOC_CLEAN gif
  !insertmacro ASSOC_CLEAN heic
  !insertmacro ASSOC_CLEAN heif
  !insertmacro ASSOC_CLEAN ico
  !insertmacro ASSOC_CLEAN jfif
  !insertmacro ASSOC_CLEAN jpe
  !insertmacro ASSOC_CLEAN jpeg
  !insertmacro ASSOC_CLEAN jpg
  !insertmacro ASSOC_CLEAN jxr
  !insertmacro ASSOC_CLEAN png
  !insertmacro ASSOC_CLEAN tif
  !insertmacro ASSOC_CLEAN tiff
  !insertmacro ASSOC_CLEAN webp
  !insertmacro ASSOC_CLEAN wdp
  DeleteRegKey HKCU "Software\Classes\${PROGID}"
  DeleteRegKey HKCU "${APP_KEY}"
  DeleteRegValue HKCU "Software\RegisteredApplications" "${REG_APP_NAME}"
  DeleteRegKey HKCU "${UNINST_KEY}"
  System::Call "shell32::SHChangeNotify(i 0x08000000, i 0, i 0, i 0)"
SectionEnd

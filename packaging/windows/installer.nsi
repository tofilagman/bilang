; Windows installer for Bilang, built by package.sh:
;   makensis -DVERSION=0.1.0 -DSTAGE=<app dir> -DOUTFILE=<setup.exe> -DICON=<bilang.ico> installer.nsi
Unicode true
SetCompressor /SOLID lzma
!include "MUI2.nsh"

Name "Bilang"
OutFile "${OUTFILE}"
InstallDir "$PROGRAMFILES64\Bilang"
InstallDirRegKey HKLM "Software\r3al\Bilang" "InstallDir"
RequestExecutionLevel admin
VIProductVersion "${VERSION}.0"
VIAddVersionKey "ProductName" "Bilang"
VIAddVersionKey "ProductVersion" "${VERSION}"
VIAddVersionKey "FileVersion" "${VERSION}"
VIAddVersionKey "FileDescription" "Bilang installer: a math adventure for kids"
VIAddVersionKey "LegalCopyright" "MIT License"

!define MUI_ICON "${ICON}"
!define MUI_UNICON "${ICON}"
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!define MUI_FINISHPAGE_RUN "$INSTDIR\bilang.exe"
!define MUI_FINISHPAGE_RUN_TEXT "Play Bilang now"
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "English"

!define UNINST "Software\Microsoft\Windows\CurrentVersion\Uninstall\Bilang"

Section "Install"
  SetOutPath "$INSTDIR"
  ; replace the previous version's files outright (DLL names change between GTK releases)
  RMDir /r "$INSTDIR\share"
  RMDir /r "$INSTDIR\lib"
  Delete "$INSTDIR\*.dll"
  File /r "${STAGE}\*.*"
  WriteUninstaller "$INSTDIR\uninstall.exe"
  CreateShortcut "$SMPROGRAMS\Bilang.lnk" "$INSTDIR\bilang.exe"
  CreateShortcut "$DESKTOP\Bilang.lnk" "$INSTDIR\bilang.exe"
  WriteRegStr HKLM "Software\r3al\Bilang" "InstallDir" "$INSTDIR"
  WriteRegStr HKLM "${UNINST}" "DisplayName" "Bilang"
  WriteRegStr HKLM "${UNINST}" "DisplayVersion" "${VERSION}"
  WriteRegStr HKLM "${UNINST}" "Publisher" "tofi lagman"
  WriteRegStr HKLM "${UNINST}" "DisplayIcon" "$INSTDIR\bilang.exe"
  WriteRegStr HKLM "${UNINST}" "URLInfoAbout" "https://github.com/tofilagman/bilang"
  WriteRegStr HKLM "${UNINST}" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegDWORD HKLM "${UNINST}" "NoModify" 1
  WriteRegDWORD HKLM "${UNINST}" "NoRepair" 1
SectionEnd

; children's progress in %APPDATA%\r3al\Bilang stays
Section "Uninstall"
  Delete "$SMPROGRAMS\Bilang.lnk"
  Delete "$DESKTOP\Bilang.lnk"
  RMDir /r "$INSTDIR"
  DeleteRegKey HKLM "${UNINST}"
  DeleteRegKey HKLM "Software\r3al\Bilang"
SectionEnd

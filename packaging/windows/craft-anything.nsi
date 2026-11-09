; Craft Anything — NSIS installer (per-machine, x64).
;
; Built by .github/workflows/windows-release.yml after packaging/windows/package.ps1:
;   makensis /DVERSION=0.4.0 /DDISPLAYVERSION=0.4.0 /DBINDIR=<stage> /DROOTDIR=<repo>
;            /DOUTFILE=<setup.exe> [/DHASCLI=1] packaging\windows\craft-anything.nsi
; <stage> holds the release exes (plus OFL-*.txt, copied there by the workflow).
; Mirrors packaging/windows/craft-anything.wxs: install location, shortcuts, registry.
; The installer UI is bilingual (English/Arabic), picked from the system locale.

!ifndef VERSION
  !error "VERSION is required (e.g. /DVERSION=0.4.0)"
!endif
!ifndef DISPLAYVERSION
  !define DISPLAYVERSION "${VERSION}"
!endif
!ifndef BINDIR
  !error "BINDIR is required (the staged exes)"
!endif
!ifndef ROOTDIR
  !error "ROOTDIR is required (the repo root)"
!endif
!ifndef OUTFILE
  !error "OUTFILE is required (the setup exe to write)"
!endif

!include "MUI2.nsh"
!include "LogicLib.nsh"
!include "x64.nsh"

Name "Craft Anything"
OutFile "${OUTFILE}"
InstallDir "$PROGRAMFILES64\Craft Anything"
InstallDirRegKey HKLM "Software\CraftAnything" "InstallDir"
RequestExecutionLevel admin
Unicode True

VIProductVersion "${VERSION}.0"
VIAddVersionKey "ProductName" "Craft Anything"
VIAddVersionKey "CompanyName" "Learning Machines LLC"
VIAddVersionKey "FileDescription" "Craft Anything setup"
VIAddVersionKey "FileVersion" "${VERSION}"
VIAddVersionKey "ProductVersion" "${DISPLAYVERSION}"

!define MUI_ABORTWARNING
!define MUI_ICON "${ROOTDIR}\assets\app-icon\craft-anything.ico"
!define MUI_UNICON "${ROOTDIR}\assets\app-icon\craft-anything.ico"

LangString FinishRun ${LANG_ENGLISH} "Run Craft Anything"
LangString FinishRun ${LANG_ARABIC} "تشغيل Craft Anything"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!define MUI_FINISHPAGE_RUN "$INSTDIR\craft-anything.exe"
!define MUI_FINISHPAGE_RUN_TEXT "$(FinishRun)"
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"
!insertmacro MUI_LANGUAGE "Arabic"

Function .onInit
  ${IfNot} ${RunningX64}
    MessageBox MB_OK|MB_ICONSTOP "Craft Anything needs 64-bit Windows."
    Abort
  ${EndIf}
  !insertmacro MUI_LANGDLL_DISPLAY
FunctionEnd

Section "Craft Anything" Main
  SectionIn RO
  SetRegView 64
  SetOutPath "$INSTDIR"
  File "${BINDIR}\craft-anything.exe"
  !ifdef HASCLI
  File "${BINDIR}\craft-anything-cli.exe"
  !endif
  File /nonfatal "${ROOTDIR}\README.md"
  File "${ROOTDIR}\LICENSE-MIT"
  File "${ROOTDIR}\LICENSE-APACHE"
  File /nonfatal "${BINDIR}\OFL-*.txt"
  WriteUninstaller "$INSTDIR\uninstall.exe"
  CreateDirectory "$SMPROGRAMS\Craft Anything"
  CreateShortcut "$SMPROGRAMS\Craft Anything\Craft Anything.lnk" "$INSTDIR\craft-anything.exe"
  CreateShortcut "$SMPROGRAMS\Craft Anything\Uninstall.lnk" "$INSTDIR\uninstall.exe"
  WriteRegStr HKLM "Software\CraftAnything" "InstallDir" "$INSTDIR"
  WriteRegStr HKLM "Software\CraftAnything" "Version" "${DISPLAYVERSION}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\CraftAnything" "DisplayName" "Craft Anything"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\CraftAnything" "DisplayVersion" "${DISPLAYVERSION}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\CraftAnything" "Publisher" "Learning Machines LLC"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\CraftAnything" "DisplayIcon" "$INSTDIR\craft-anything.exe,0"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\CraftAnything" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\CraftAnything" "NoModify" 1
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\CraftAnything" "NoRepair" 1
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\App Paths\craft-anything.exe" "" "$INSTDIR\craft-anything.exe"
SectionEnd

Section "Uninstall"
  SetRegView 64
  Delete "$INSTDIR\craft-anything.exe"
  Delete "$INSTDIR\craft-anything-cli.exe"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\LICENSE-MIT"
  Delete "$INSTDIR\LICENSE-APACHE"
  Delete "$INSTDIR\OFL-*.txt"
  Delete "$INSTDIR\uninstall.exe"
  Delete "$SMPROGRAMS\Craft Anything\Craft Anything.lnk"
  Delete "$SMPROGRAMS\Craft Anything\Uninstall.lnk"
  RMDir "$SMPROGRAMS\Craft Anything"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\CraftAnything"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\App Paths\craft-anything.exe"
  DeleteRegKey HKLM "Software\CraftAnything"
  RMDir "$INSTDIR"
SectionEnd

; Inno Setup Script for Tailorbird Desktop Application
; Generates Tailorbird-Setup-v0.1.0.exe

#define MyAppName "Tailorbird"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "Joel Stransky"
#define MyAppURL "https://github.com/joelstransky/Tailorbird"
#define MyAppExeName "tailorbird.exe"

[Setup]
AppId={{9B7F3652-3A68-4BE9-BA43-E6E41D29D47C}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DisableProgramGroupPage=yes
; Per-user install allows installation without administrator privileges
PrivilegesRequired=lowest
OutputDir=..\dist
OutputBaseFilename=Tailorbird-Setup-v{#MyAppVersion}
SetupIconFile=..\src\assets\icon.ico
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
UninstallDisplayIcon={app}\{#MyAppExeName}
CloseApplications=yes
RestartApplications=no

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
; The compiled release binary
Source: "..\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
; Application icon
Source: "..\src\assets\icon.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\icon.ico"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\icon.ico"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

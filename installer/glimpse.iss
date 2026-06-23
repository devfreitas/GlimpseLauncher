#define MyAppName "GlimpseLauncher"
#define MyAppVersion "0.6.0"
#define MyAppPublisher "DevFreitas"
#define MyAppURL "https://github.com/devfreitas/GlimpseLauncher"
#define MyAppExeName "glimpse_launcher.exe"

[Setup]
AppId={{CE1AF328-AD3F-4F64-AFA5-9F79A2F5C355}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}

AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}

PrivilegesRequired=lowest

DefaultDirName={localappdata}\Programs\{#MyAppName}
DefaultGroupName={#MyAppName}

ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

DisableProgramGroupPage=yes

WizardStyle=modern dynamic

Compression=lzma2
SolidCompression=yes

OutputDir=Output
OutputBaseFilename=GlimpseLauncher-{#MyAppVersion}-Setup

SetupIconFile=..\public\icone.ico

UninstallDisplayIcon={app}\{#MyAppExeName}

VersionInfoVersion={#MyAppVersion}
VersionInfoCompany={#MyAppPublisher}
VersionInfoDescription=Glimpse Launcher Installer
VersionInfoCopyright=Copyright (c) DevFreitas

CloseApplications=yes
RestartApplications=no

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; Flags: unchecked

[Files]
Source: "..\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\public\icone.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Glimpse Launcher"; \
    Filename: "{app}\{#MyAppExeName}"; \
    IconFilename: "{app}\icone.ico"

Name: "{autodesktop}\Glimpse Launcher"; \
    Filename: "{app}\{#MyAppExeName}"; \
    IconFilename: "{app}\icone.ico"; \
    Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; \
    Description: "Launch Glimpse Launcher"; \
    Flags: nowait postinstall skipifsilent
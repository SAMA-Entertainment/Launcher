# MikuniLauncher

Official __**Mikuni**__ Launcher built using *Rust* and *Javascript*.
This launcher provides auto-update functionality to the game with additional installation information. Please check out
our project at [the Mikuni Website](https://mikuni.me/ "Official Mikuni Website").
This project uses Tauri, a lightweight web framework for building desktop apps.

## Auto-updater 
The auto-updater checks the version information located at <https://mikuni.me/auto_update_check.json> to decide if an
update is available. Then it downloads the version as a ZIP file and automatically unzip it and copies its's files to
the correct installation folder.

## Building

### Building the launcher

You must install Rust and NodeJS to build this project. You must also install Visual Studio Code Tools 2019
(required by Tauri.). 

To serve the launcher locally run:
```shell
npm run tauri-dev
```

To build the executable run:
```shell
npm run tauri-build
```

### Building the installer

To build the installer you must install [Inno Setup](https://jrsoftware.org/isdl.php)
and download the [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/).
Inside Inno Setup configure SignTool and then run:
```shell
iscc installer/MikuniInstaller.iss
```


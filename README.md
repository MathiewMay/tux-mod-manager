# Tux Mod Manager
TMM is a Linux native mod manager made with the Tauri toolkit. It can install, load, remove and deploy mods for both Linux native and WINE games.

## TMM Roadmap
- [x] Move the current mod manager logic to rust
- [x] Implement a OFS (Overlay File System, similar to VFS from MO2)
- [x] Make remove button actually work.
- [x] Add visual indication that a mod is installing
- [x] Rewrite mod_manager.rs using the steamlocate lib instead of searching for steam directories and games intrusively
- [x] Only create instance config files if a game is supported
- [x] Read a game's config from an already existing config file
- [x] Support storing game files in a user defined directory
- [ ] Download manager for directly downloading mods from websites (e.g. Nexusmods)
    - [x] Front-End Design
    - [x] The actual file download
    - [x] Putting the file in the correct location
    - [x] Input download URL in Front-End
    - [ ] Implementing some kind of handshake with the Nexusmods API to allow seamless downlaods via the `Mod Manager Download` button on their website
    - [x] Displaying the current downloads in the Front-End
    - [ ] Improve Downloads Display:
        - [ ] Display ETA
        - [ ] Download Speed
        - [ ] Make `Install`, `Remove` and `Cancel` buttons actually work
- [ ] Move `known_path_extensions.json` and `supported_games.json` into tauri's distribution directory, so they are bundled when building the application and aren't required to be in `%XDG_CONFIG_DIR%/tmm_stage/` 
- [ ] Implement a game launcher for native and proton games (for the OFS)
- [ ] Implement a per-game load order
- [ ] Implement mod profiles
- [ ] Create cli commands, example to launch a game from steam with a specifc profile without having to use the mod manager

## Current indev issues
- [ ] For games with path extensions, some mods may have the extension folder already in their archive, some may not. This means that sometimes a mod has the extension folder, sometimes it doesn't. In the end this would resoult in the Game not reading some mods, because they're essentially in the wrong folder (e.g. `data/data/`)
- [ ] Download manager does not yet currently work entirely

## Dev environment
If you want to help with the development of Tux Mod Manager you will need to setup a dev environment this is how you do that.
(If you would rather just use the software without helping development please wait for a "stable" release)

Install dev environment
```
git clone https://github.com/MathiewMay/tux-mod-manager
cd ./tux-mod-manager
npm install
```

Run dev environment
```
npm run tauri dev
```

## Dependancies

Rust v.1.66.1+ (https://www.rust-lang.org/tools/install)

### Ubuntu
```
sudo apt install npm libdbus-1-dev pkg-config libpango1.0-dev librust-gdk-dev libjavascriptcoregtk-4.0-dev libarchive-dev libsoup2.4-dev libwebkit2gtk-4.0-dev
```

### Fedora/Red Hat (NOT COMPLETE I don't have a Fedora system right now)

```
sudo dnf install dbus-devel pkgconf-pkg-config
```
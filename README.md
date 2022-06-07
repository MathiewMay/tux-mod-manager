# Tux Mod Manager
TMM is a Linux native mod manager made with the Tauri toolkit. It can install, load, remove and deploy mods for both Linux native and WINE games.

## TMM Roadmap
- [x] Move the current mod manager logic to rust
- [x] Implement a OFS (Overlay File System, similar to VFS from MO2)
- [x] Make remove button actually work.
- [x] Add visual indication that a mod is installing
- [x] Rewrite mod_manager.rs using the steamlocate lib instead of searching for steam directories and games intrusively
- [ ] Download manager for directly downloading mods from websites (e.g. Nexusmods)
    - [x] Front-End Design
    - [x] The actual file download
    - [ ] Putting the file in the correct location
    - [ ] Displaying the current downloads in the Front-End
- [ ] Implement a game launcher for native and proton games (for the OFS)
- [ ] Implement a per-game load order
- [ ] Implement mod profiles
- [ ] Create cli commands, example to launch a game from steam with a specifc profile without having to use the mod manager

## Current indev issues
- [ ] Some mods may have a `data` folder already in their archive, some may not. This leads to a double up of the data folder, which would prevent the mod from loading.
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

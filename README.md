# Tux Mod Manager
TMM is a Linux native mod manager made with the Tauri toolkit, it can, install, load, remove, and deploy mods for both linux native and wine games.

## TMM Roadmap
 - 🟢 Move the current mod manager logic to rust
 - 🟢 Implement a OFS (Overlay File System, similar to VFS from MO2)
 - 🟢 Make remove button actually work.
 - 🟢 Add visual indication that a mod is installing
 - 🟢 Rewrite mod_manager.rs using the steamlocate lib instead of searching for steam directories and games intrusively
 - 🔴 Implement a game launcher for native and proton games (for the OFS)
 - 🔴 Implement a per-game load order
 - 🔴 Implement mod profiles
 - 🔴 Create cli commands, example to launch a game from steam with a specifc profile without having to use the mod manager

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

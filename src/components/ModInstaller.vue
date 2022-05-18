<script>
import { fs, path, dialog } from '@tauri-apps/api'
import { invoke } from '@tauri-apps/api/tauri'

export default {
  props: ['selected_game'],
  methods: {
    async installMod() {
      const appDir = await path.appDir()
      dialog.open().then((file) => {
        const fileName = file.split('/')[file.split('/').length-1]
        const appGameDir = appDir+"games/"+this.selected_game.name+"/mods/"+fileName.split('.')[0]
        invoke('uncompress', { filePath: file, targetPath: appGameDir })
      })
    }
  }
}
</script>

<template>
<button @click="installMod">Install Mods</button>
</template>

<style>
</style>
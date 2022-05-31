<script>
import { path, dialog } from '@tauri-apps/api'
import { invoke } from '@tauri-apps/api/tauri'

import supported_games_json from '../assets/supported-games.json'

export default {
  props: ['selected_game'],
  data() {
    return {
      supported_games: supported_games_json,
    };
  },
  methods: {
    async installMod() {
      const stageDir = await path.homeDir()+".config/tmm_stage/"
      dialog.open().then((file) => {
        const fileFullName = file.split('/')[file.split('/').length-1]
        const fileExtension = fileFullName.split('.')[fileFullName.split('.').length-1]
        const fileName = fileFullName.split('.')[0]
        if(fileExtension == "zip" || fileExtension == "rar" || fileExtension == "7z"){
          const appGameDir = stageDir+"games/"+this.selected_game.name+"/mods/"+fileName+supported_games_json[this.selected_game.name].extensionsPath['**']
          invoke('uncompress', { filePath: file, targetPath: appGameDir }).then((result) => {
            this.$emit('on-mod-installed', fileName)
          })
        }else{
          dialog.message("File format mismatch.")
        }

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
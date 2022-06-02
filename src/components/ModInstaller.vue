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
      dialog.open().then((file) => {
        const stageGameDir = this.selected_game.stage_path;
        const fileFullName = file.split('/')[file.split('/').length-1]
        const fileExtension = fileFullName.split('.')[fileFullName.split('.').length-1]
        const fileName = fileFullName.split('.')[0]
        if(fileExtension == "zip" || fileExtension == "rar" || fileExtension == "7z"){
          const modStageDir = stageGameDir+"/"+fileName+supported_games_json[this.selected_game.appid].extensionsPath['**']
          console.log(modStageDir);
          invoke('uncompress', { filePath: file, targetPath: modStageDir }).then(()=>{this.$emit('on-mod-installed', fileName)})
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
<li class="mod-list" v-for="(mod) in mods" :key="mod">
  <Mod ref="mod_ref" :selected_game="selected_game" :mod_name="mod.name" :tmm_mod_path="mod.path"/>
</li>
</template>

<style>
</style>
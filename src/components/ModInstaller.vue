<script>
import { ref } from '@vue/reactivity'
import { dialog } from '@tauri-apps/api'
import { invoke } from '@tauri-apps/api/tauri'

import Mod from './Mod.vue'
import supported_games_json from '../assets/supported-games.json'

export default {
  props: ['selected_game'],
  data() {
    return {
      supported_games: supported_games_json,
    };
  },
  components: {
    Mod,
  },
  setup() {
    const initialState = {}
    const mods = ref({...initialState})
    function resetMods(){
      mods.value = {...initialState}
    }
    return {mods, resetMods} 
  },
  methods: {
    async installMod() {
      dialog.open({filters: [{ extensions: ['zip', 'rar', '7z'], name: "Archives" }]}).then((file) => {
        const stageGameDir = this.selected_game.stage_path;
        const fileFullName = file.split('/')[file.split('/').length-1]
        const fileExtension = fileFullName.split('.')[fileFullName.split('.').length-1]
        const fileName = fileFullName.split('.')[0]
        const modStageDir = stageGameDir+"/"+fileName+supported_games_json[this.selected_game.appid].extensionsPath['**']
        this.mods[fileName] = {name: fileName, path: modStageDir}
        invoke('uncompress', { filePath: file, targetPath: modStageDir }).then(()=>{
          this.$emit('on-mod-installed', fileName)
          delete this.mods[fileName]
        })
      })
    }
  }
}
</script>

<template>
<div class="installer">
  <div class="install-mod-order">
    <li class="install-mod-list" v-for="(mod) in mods" :key="mod">
      <Mod :selected_game="selected_game" :mod="mod" :installing="true"/>
    </li>
  </div>
  <button class="install-button" @click="installMod">Install Mods</button>
</div>

</template>

<style>
.installer {
  width: 865px;
}
.install-mod-order {
  width: 100%;
  max-height: 500px;
  overflow: hidden;
  overflow-y: scroll;
}
.install-mod-list { 
  margin: 5px;
  list-style: none;
}
.install-button {
  position: fixed;
  margin-bottom: 5px;
  margin-right: 5px;
  bottom: 0%;
  right: 0%;
  width: 100px;
  height: 30px;
}
</style>
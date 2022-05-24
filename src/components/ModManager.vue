<script>
import { path } from '@tauri-apps/api'
import { ref } from '@vue/reactivity'
import { invoke } from '@tauri-apps/api/tauri'

import Mod from './Mod.vue'
import ModInstaller from './ModInstaller.vue'

import Mixins from '../Mixins';
import supported_games_json from '../assets/supported-games.json'

export default {
  mixins: [Mixins],
  data() {
    return {
      supported_games: supported_games_json,
    };
  },
  props: ['selected_game'],
  components: {
    Mod,
    ModInstaller
  },
  setup() {
    const initialState = {}
    const mods = ref({ ...initialState })
    function resetMods(){
      mods.value = {...initialState}
    }
    return {mods, resetMods}
  },
  methods: {
    async refreshModList(){
      this.resetMods()
      const appPath = await path.appDir()
      const selectedGamePath = appPath+"games/"+this.selected_game.name+"/mods/"
      const selectedGameModsEntrys = await Mixins.methods.getDirectorysFromPath(selectedGamePath)
      selectedGameModsEntrys.forEach(modEntry => {
        this.mods[modEntry.name] = modEntry
      })
    },

    async deployMods(){
      if(this.$refs.mod_ref != undefined){
        const modList = this.$refs.mod_ref
        for(var i=0; i<modList.length; i++){
          const mod = modList[i]
          const json = {path: mod.tmm_mod_path}      
          invoke('deploy_mod', { mod: JSON.stringify(json), deploy: mod.$refs.mod_enabled.checked })
        }
      }
    },

    async copyDir(from_path, to_path){
      const fromDir = await fs.readDir(from_path)
      for(var i=0; i<fromDir.length; i++){
        const file = fromDir[i]
        if(!await Mixins.methods.pathExists(to_path+file.name)){
          if(!file.children){
            fs.copyFile(file.path, to_path+file.name)
          }else{
            fs.createDir(to_path+file.name)
            this.copyDir(file.path, to_path+file.name+"/")
          }
        }
      }
    },
    async removeMod(tmm_path, game_path){
      const modDir = await fs.readDir(tmm_path)
      for(var i=0; i<modDir.length; i++){
        const file = modDir[i]
        if(await Mixins.methods.pathExists(game_path+file.name)){
          if(!file.children){
            fs.removeFile(game_path+file.name)
          }else{
            this.removeMod(file.path, game_path+file.name+"/")
          }
        }
      }
    }
  }
}
</script>

<template>
<div class="mod-manager">
  <div class="mod-order">
    <li class="mod-list" v-for="(mod) in mods" :key="mod">
      <Mod ref="mod_ref" :mod_name="mod.name" :tmm_mod_path="mod.path"/>
    </li>
  </div>
  <div class="separator-bottom"></div>
  <div class="mod-install">
      <ModInstaller :selected_game="selected_game" @on-mod-installed="refreshModList"/>
  </div>
</div>
</template>

<style scoped>
.mod-manager {
  display: inline-flex;
  width: 865px;
}
.mod-order {
  width: 100%;
  max-height: 500px;
  overflow: hidden;
  overflow-y: scroll;
}
.mod-list { 
  margin: 5px;
  list-style: none;
}
.separator-bottom{
    color: black;
    background-color: rgba(255,255,255,8%);
    width: 864px;
    height: 1px;
    position: absolute;
    margin-left: 1px;
    top: 500px;
}
.mod-install {
  position: absolute;
  top: 500px;
}
</style>
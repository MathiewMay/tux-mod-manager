<script>
import { ref } from '@vue/reactivity'
import { invoke } from '@tauri-apps/api/tauri'
//import { fs } from '@tauri-apps/api'

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
      const modsEntrys = await invoke('get_mods_name', {gameName: this.selected_game.name})
      modsEntrys.forEach(modEntry => {
        const modJson = JSON.parse(modEntry)
        this.mods[modJson.name] = modJson
      })
    },

    async deployMods(){
      if(this.$refs.mod_ref != undefined){
        const modList = this.$refs.mod_ref
        let enabledMod = []
        for(var i=0; i<modList.length; i++){
          const mod = modList[i]
          const json = {name: mod.mod_name, path: mod.tmm_mod_path}
          if(mod.$refs.mod_enabled.checked)
            enabledMod.push(JSON.stringify(json))     
          //
          // Deploying mods needs to be rewrittin in rust, with OFS until then nothing.
          //
          /*if(mod.$refs.mod_enabled.checked){
            this.copyDir(mod.tmm_mod_path, this.selected_game.path+supported_games_json[this.selected_game.name].extensionsPath['**'])
          }else{
            this.removeMod(mod.tmm_mod_path, this.selected_game.path+supported_games_json[this.selected_game.name].extensionsPath['**'])
          }*/
        }
        const jsonGame = JSON.stringify({name: this.selected_game.name, path: this.selected_game.path})
        invoke('deploy', { mods: enabledMod, game: jsonGame })
      }
    },

    /*async copyDir(from_path, to_path){
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
    }*/
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
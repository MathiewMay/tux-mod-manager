<script>
import { ref } from '@vue/reactivity'
import { invoke } from '@tauri-apps/api/tauri'

import Mod from './Mod.vue'
import ModInstaller from './ModInstaller.vue'

import supported_games_json from '../assets/supported-games.json'

export default {
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
    const mods = ref({...initialState})
    function resetMods(){
      mods.value = {...initialState}
    }
    return {mods, resetMods}
  },
  methods: {
    async refreshModList(){
      this.resetMods()
      const modsEntrys = await invoke('get_mods', {game: this.selected_game})
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
          const mod_ref = modList[i]
          if(mod_ref.$refs.mod_enabled.checked)
            enabledMod.push(mod_ref.mod)
        }
        invoke('deploy', { mods: enabledMod, game: this.selected_game })
      }
    },
  }
}
</script>

<template>
<div class="mod-manager">
  <div class="mod-order">
    <li class="mod-list" v-for="(mod) in mods" :key="mod">
      <Mod ref="mod_ref" :selected_game="selected_game" :mod="mod" :installing="'false'"/>
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
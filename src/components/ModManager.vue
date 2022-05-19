<script>
import { fs, path } from '@tauri-apps/api'
import { ref } from '@vue/reactivity'

import Mod from './Mod.vue'
import ModInstaller from './ModInstaller.vue'

import Mixins from '../Mixins';

export default {
  mixins: [Mixins],
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
    }
  }
}
</script>

<template>
<div class="mod-manager">
  <div class="mod-order">
    <li class="mod-list" v-for="(mod) in mods" :key="mod">
      <Mod :mod_name="mod.name" />
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
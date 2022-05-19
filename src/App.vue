<script>
import { fs, path } from '@tauri-apps/api'
import { ref } from '@vue/reactivity'

import SideBar from './components/SideBar.vue'
import ModManager from './components/ModManager.vue'

import supportedGames from './assets/supported-games.json'

import Mixins from './Mixins';

appDirectoryCheck()
async function appDirectoryCheck() {
  const appPath = await path.appDir()
  if(!await Mixins.methods.pathExists(appPath))
    await fs.createDir(appPath)
    if(!await Mixins.methods.pathExists(appPath+"games"))
      await fs.createDir(appPath+"games")
}

export default {
  mixins: [Mixins],
  data() {
    return {
      supported_games: supportedGames,
    };
  },
  setup() {
    const selected_game = ref({})
    return {selected_game}
  },
  methods: {
    async gameSelected(gameEntry) {
      const appDir = await path.appDir()
      const appGameDir = appDir+"games/"+gameEntry.name
      this.selected_game = gameEntry
      if(!await Mixins.methods.pathExists(appGameDir))
        fs.createDir(appGameDir)
        if(!await Mixins.methods.pathExists(appGameDir+"/mods"))
        fs.createDir(appGameDir+"/mods")
    }
  },
  components: {
    SideBar,
    ModManager
  }
}

</script>

<template>
  <SideBar @on-game-selected="gameSelected"/>
  <ModManager v-if="selected_game.name" :selected_game="selected_game"/>
</template>

<style>
*{
  -webkit-user-select: none;
  user-select: none;
}
body {
  margin: 0px;
  background-color: #121212;
}
button {
  border: none;
  border-radius: 5px;
  background-color: rgba(255,255,255,9%);
  color: rgba(255,255,255,100%);
}
button:hover {
  cursor: pointer;
  background-color: rgba(255,255,255,12%);
}
</style>

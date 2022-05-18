<script>

import { fs, path } from '@tauri-apps/api'

import SideBar from './components/SideBar.vue'
import ModManager from './components/ModManager.vue'

import supportedGames from './assets/supported-games.json'
import { ref } from '@vue/reactivity'

appPathCheck()
async function appPathCheck() {
  const appPath = await path.appDir()
  if(!await pathExists(appPath))
    await fs.createDir(appPath)
    if(!await pathExists(appPath+"games"))
      await fs.createDir(appPath+"games")
}

async function pathExists(path) {
  const pathDirExists = await fs.readDir(path).catch((reason) => {return true})
  if(pathDirExists != true){
    return true
  }else{
    return false
  }
}

export default {
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
    async pathExists(path) {
      const pathDirExists = await fs.readDir(path).catch((reason) => {return true})
      if(pathDirExists != true){
        return true
      }else{
        return false
      }
    },

    async gameSelected(gameEntry) {
      const appDir = await path.appDir()
      const appGameDir = appDir+"games/"+gameEntry.name
      this.selected_game = gameEntry
      if(!await pathExists(appGameDir))
        fs.createDir(appGameDir)
        if(!await pathExists(appGameDir+"/mods"))
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
  <SideBar @game-selected="gameSelected"/>
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

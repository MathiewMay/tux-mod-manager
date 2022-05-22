<script>
import { fs, path } from '@tauri-apps/api'
import { reactive } from '@vue/reactivity'

import Mixins from '../Mixins';
import supported_games_json from '../assets/supported-games.json'

export default {
  mixins: [Mixins],
  data() {
    return {
      supported_games: supported_games_json,
    };
  },
  setup() {
    const games = reactive({})
    return {games}
  },
  methods: {
    async scanGames() {
      let steamGamesEntry = []
      const homeDir = await path.homeDir()
      const steamLocalPath = homeDir+".local/share/Steam/steamapps/common/"
      const steamFlatpakPath = homeDir+".var/app/com.valvesoftware.Steam/data/Steam/steamapps/common/"
      const steamPaths = [steamLocalPath,steamFlatpakPath]
      for(var i=0; i<steamPaths.length; i++){
        if(await Mixins.methods.pathExists(steamPaths[i])){
          const localSteamPath = steamPaths[i]
          const localGameEntrys = await Mixins.methods.getDirectorysFromPath(localSteamPath)
          steamGamesEntry = steamGamesEntry.concat(localGameEntrys)
        }
      }

      const mntDir = await fs.readDir('/mnt/')
      for(var i=0; i<mntDir.length; i++){
        const mntSteamPath = mntDir[i].path+'/SteamLibrary/steamapps/common'
        if(await Mixins.methods.pathExists(mntSteamPath)){
          const mntGameEntrys = await Mixins.methods.getDirectorysFromPath(mntSteamPath)
          steamGamesEntry = steamGamesEntry.concat(mntGameEntrys)
        }
      }

      steamGamesEntry.forEach(entry => {
        if(this.supported_games[entry.name]){
          this.games[entry.name] = entry
        }
      })
      this.$emit('on-scan-games')
    },

    sendDeployMods() {
      this.$emit('deploying-mods')
    },
    selectNewGame(e, gameEntry){
      const gameButton = e.target 
      const buttonList = this.$el.querySelectorAll('.game-list li button')
      buttonList.forEach(elem => {
        elem.className = ""
      })
      gameButton.className = "active"
      this.$emit('on-game-selected', gameEntry)
    }
  }
}
</script>

<template>
<div class="side-bar">
  <button class="scan-games-button" @click="scanGames()">Scan games</button>
  <div class="game-list">
    <li v-for="(game) in games" :key="game">
      <button @click="selectNewGame($event, game)">{{ game.name }}</button>
    </li>
  </div>
  <div class="options-bottom">
    <button class="settings-button">···</button>
    <button class="deploy-button" @click="sendDeployMods()">Deploy</button>
  </div>
  <div class="separator-right"></div>
</div>
</template>
  
<style scoped>
.side-bar {
  float: left;
  width: 215px;
  max-height: 720px;
  overflow: hidden;
}
.scan-games-button {
  width: 175px;
  height: 50px;
  margin-left: 20px;
  margin-top: 15px;
  margin-bottom: 10px;
  margin-right: 20px;
  font-size: 20px;
}
.game-list {
  background-color: rgba(255,255,255,7%);
  border-radius: 5px;
  padding: 1px;
  margin-left: 20px;
  margin-right: 20px;
  min-height: 596px;
  max-height: 596px;
  overflow: hidden;
  overflow-y: scroll
}
.game-list li {
  list-style: none;
}
.game-list button {
  text-align: left;
  display: block;
  width: 165px;
  margin: auto;
  margin-top: 5px;
  margin-bottom: 5px;
  background-color: rgba(255,255,255,0%);
  color: rgba(255,255,255,100%);
}
.game-list button:hover {
  background-color: rgba(255,255,255,4%);
  color: rgba(255,255,255,100%);
}
.game-list .active, .game-list .active:hover {
  background-color: rgba(255,255,255,9%);
  color: rgba(255,255,255,100%);
}
.options-bottom {
  margin-top: 7px;
  margin-left: 20px;
}
.options-bottom button {
  height: 35px;
  margin-bottom: 4px;
  font-size: 16px
}
.options-bottom i {
  font-size: 16px;
}
.settings-button {
  width: 35px;
}
.deploy-button {
  width: 115px;
  margin-left: 25px;
  margin-right: 20px;
}
.separator-right {
    color: black;
    background-color: rgba(255,255,255,8%);
    width: 1px;
    height: 720px;
    float: right;
    margin-left: 215px;
    position: absolute;
    top: 0;
    left: 0;
}
</style>
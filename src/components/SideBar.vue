<script>
import { reactive } from '@vue/reactivity'
import { invoke } from '@tauri-apps/api/tauri'

import supported_games_json from '../assets/supported-games.json'

export default {
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
      await invoke('scan_games').then((entrys) => {
        entrys.forEach(element => {
          let game = JSON.parse(element)
          if(this.supported_games[game.appid]){
            this.games[game.appid] = game
          }
        })
      })
      this.$emit('on-scan-games')
    },

    sendDeployMods() {
      this.$emit('deploying-mods')
    },

    selectNewGame(e, gameEntry){
      const gameButton = e.target 
      const buttonList = this.$refs.game_ref
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
      <button ref="game_ref" @click="selectNewGame($event, game)">{{ game.name }}</button>
    </li>
  </div>
  <div class="options-bottom">
    <button class="run-button">Run</button>
    <button class="deploy-button" @click="sendDeployMods()">Deploy</button>
  </div>
  <!-- <div class="separator-right"></div> -->
</div>
</template>
  
<style scoped lang="scss">
.side-bar {
  min-width: 250px;
  width: 250px;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border-right: 1px solid rgba(255,255,255,8%);
  padding: 10px;
  .scan-games-button {
    height: 50px;
    font-size: 20px;
    margin: 0;
    margin-bottom: 10px;
  }
  .game-list {
    flex: 1 0 auto;
    background-color: rgba(255,255,255,7%);
    border-radius: 5px;
    overflow: hidden;
    overflow-y: scroll;
    li {
      list-style: none;
      width: 100%;
      display: flex;
      flex-direction: row;
      border-bottom: 1px solid rgba(255,255,255,8%);
      &:last-child {
        border-bottom: none;
      }
      button {
        text-align: center;
        display: block;
        width: 100%;
        margin: 5px;
        background-color: rgba(255,255,255,0%);
        color: rgba(255,255,255,100%);
        &:hover {
          background-color: rgba(255,255,255,4%);
        }
        &.active {
          background-color: rgba(255,255,255,9%);
        }
      }
    }
  }
  .options-bottom {
    margin-top: 10px;
    margin-inline: auto;
    button {
      margin: 0;
      padding: 5px;
      padding-inline: 15px;
      margin-right: 10px;
      &:last-child {
        margin-right: 0;
      }
    }
  }
}
/* .separator-right {
  color: black;
  background-color: rgba(255,255,255,8%);
  width: 1px;
  height: 720px;
  float: right;
  margin-left: 215px;
  position: absolute;
  top: 0;
  left: 0;
} */
</style>
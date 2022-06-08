<script>
import { ref } from '@vue/reactivity'

import SideBar from './components/SideBar.vue'
import MainPanel from './components/MainPanel.vue'

export default {
  setup() {
    const selected_game = ref({})
    function resetSelectedGame(){
      selected_game.value = {}
    }
    return {selected_game, resetSelectedGame}
  },
  methods: {
    newScanGames(){
      this.resetSelectedGame()
    },
    deployMods(){
      this.$refs.main_panel.deployMods();
    },
    async newGameSelected(game) {
      this.selected_game = game;
      this.$refs.main_panel.newGameSelected(game);
    }
  },
  components: {
    SideBar,
    MainPanel
  }
}
</script>

<template>
  <div class="flex-container">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.1.1/css/all.min.css" integrity="sha512-KfkfwYDsLkIlwQp6LFnl8zNdLGxu9YAA1QvwINks4PhcElQSvqcyVLLD9aMhXd13uQjoXtEKNosOWaZqXgel0g==" crossorigin="anonymous" referrerpolicy="no-referrer" />
    <SideBar @deploy-mods="deployMods()" @on-game-selected="newGameSelected" @on-scan-games="newScanGames"/>
    <MainPanel ref="main_panel" :selected_game="selected_game"/>
  </div>
</template>

<style lang="scss" scoped>
.flex-container {
  display: flex;
  flex-direction: row;
  height: 100%;
}
</style>

<style>
*{
  -webkit-user-select: none;
  user-select: none;
  box-sizing: border-box;
}
html {
  height: 100%;
}
body {
  margin: 0px;
  height: 100%;
  background-color: #121212;
}
#app {
  height: 100%;
}
button {
  border: none;
  border-radius: 5px;
  background-color: rgba(255,255,255,9%);
  color: rgba(255,255,255,100%);
  padding: 5px;
  padding-inline: 10px;
}
button:hover {
  cursor: pointer;
  background-color: rgba(255,255,255,12%);
}
</style>
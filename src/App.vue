<script>
import { ref } from '@vue/reactivity'
import { invoke } from '@tauri-apps/api/tauri'

import SideBar from './components/SideBar.vue'
import ModManager from './components/ModManager.vue'

import Mixins from './Mixins';
import supported_games_json from './assets/supported-games.json'

invoke('make_stage_directory');

export default {
  mixins: [Mixins],
  data() {
    return {
      supported_games: supported_games_json,
    };
  },
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
      if(this.$refs.mod_manager != undefined)
        this.$refs.mod_manager.deployMods()
    },
    async newGameSelected(game) {
      invoke('make_game_stage_directory', {gameName: game.name})
      this.selected_game = game
      setTimeout(() => { this.$refs.mod_manager.refreshModList() }, 1);

    }
  },
  components: {
    SideBar,
    ModManager
  }
}

</script>

<template>
  <SideBar @on-game-selected="newGameSelected" @on-scan-games="newScanGames" @deploying-mods="deployMods()"/>
  <ModManager v-if="selected_game.name" ref="mod_manager" :selected_game="selected_game"/>
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
<script>

import SideBar from './components/SideBar.vue'
import ModManager from './components/ModManager.vue'

import supportedGames from './assets/supported-games.json'
import { reactive, ref } from '@vue/reactivity'

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
    gameSelected(gameEntry) {
      this.selected_game = gameEntry
      console.log(this.selected_game)
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
  <ModManager v-if="selected_game.name"/>
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

<script>
import { ref } from '@vue/reactivity'
import { event } from '@tauri-apps/api'
import { WebviewWindow } from '@tauri-apps/api/window'

import SideBar from './components/SideBar.vue'
import MainPanel from './components/MainPanel.vue'

export default {
    setup() {                          //   Set up variables needed
        const selected_game = ref({})  // the selected_game is for any games that were found on the system
        const supported_game = ref({}) // supported_game is for a game that isn't found yet, lets the user scan for only the selected game
        function resetSelectedGame(){  // reset the selected game variables
            selected_game.value = {}
            supported_game.value = {}
        }
        const games = ref({})          // games is a list of found games
        
        // console.log(window);
        return {selected_game, resetSelectedGame, games, supported_game}
    },
    methods: {
        resetGame(){  
            this.resetSelectedGame()    // When the list of games is scanned, the selected games are reset
        },
        deployMods(){  // the deploy mods button in the side panel activates the main panel function
            this.$refs.main_panel.deployMods()  
        },
        async newGameSelected(game) {   // If you select a new game on the side panel, update the main panel with its info
            this.resetSelectedGame()
            this.selected_game = game
            this.$refs.main_panel.newGameSelected()
        },
        async newNotFoundGameSelected(game) {  // Same as above, this time changing the supported_game for the mainpanel scan
            this.resetSelectedGame()
            this.supported_game = game
        },
    },
    components: {  // After the app is in a basic working state, I would like to see the list of games in the main panel, a navigation bar on the left, and a control center bottom
        SideBar,
        MainPanel
    }
}
</script>

<template>
    <div class="flex-container">
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.1.1/css/all.min.css" integrity="sha512-KfkfwYDsLkIlwQp6LFnl8zNdLGxu9YAA1QvwINks4PhcElQSvqcyVLLD9aMhXd13uQjoXtEKNosOWaZqXgel0g==" crossorigin="anonymous" referrerpolicy="no-referrer" />
        <SideBar @deploy-mods="deployMods()" @on-game-selected="newGameSelected" @on-reset-game="resetGame" @on-not-found-game-selected="newNotFoundGameSelected" :games="games" :supported_game="supported_game" />
        <MainPanel ref="main_panel" :selected_game="selected_game" :games="games" :supported_game="supported_game" />
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
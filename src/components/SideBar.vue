<!-- TODO
 * Show a list of supported games on the side bar at all times, make it clear when they've been found or not
This will also allow users to search for one game individually instead of scanning for the whole supported games list.
It could also allow them to just add the path and other needed information manually. -->

<script>
import { reactive } from '@vue/reactivity'
import { invoke } from '@tauri-apps/api/tauri'

import supported_games from '../assets/supported-games.json'

export default {
    data() {
        return {
            supported_games 
        };
    },
    props: ['games', 'supported_game'],  // the games and supported_game variables are passed to the SideBar component
    methods: {
        selectNewGame(e, gameEntry){  // when a new game is selected, get the event info to identify which button was pressed and the game data
            const gameButton = e.target   
            const buttonList = this.$refs.game_ref
            buttonList.forEach(elem => {
                elem.classList.remove("active");
            })
            const secondButtonList = this.$refs.supported_game_ref
            secondButtonList.forEach(elem => {
                elem.classList.remove("active");
            })
            gameButton.classList.add("active");             // Remove the 'active' CSS class from all the buttons not pressed
            this.$emit('on-game-selected', gameEntry)  //  Call the newGameSelected function in App.vue passing the game data with it
        },

        selectSupportedGame(e, supportedGame){  // Same function as above, only for the games that aren't scanned yet
            const gameButton = e.target
            if (this.$refs.game_ref) { // If there isn't any scanned games yet, don't try to update their listings
                const buttonList = this.$refs.game_ref
                buttonList.forEach(elem => {
                elem.classList.remove("active");
                })
            }
            const secondButtonList = this.$refs.supported_game_ref
            secondButtonList.forEach(elem => {
                elem.classList.remove("active");
            })
            gameButton.classList.add("active");
            this.$emit('on-not-found-game-selected', supportedGame)
        },

        async scanGames() {  // when the scan games button is clicked
            await invoke('scan_games', { supportedGames: supported_games }).then((entries) => {  // Call the rust function scan_games passing the list of supported games to it
                entries.forEach(element => {
                    let game = JSON.parse(element)
                    this.games[game.appid] = game
                })
            })
            this.$emit('on-reset-game')
        }, 
    
        sendDeployMods() {  // when the deploy mods button is pressed, call the deploy mods function in App.vue
            this.$emit('deploy-mods')
        },
    }
}
</script>

<template>
<div class="side-bar">
    <button class="scan-games-button" @click="scanGames()">Scan games</button>  <!-- activates the scanGames method above-->
    <div class="game-list">  <!-- List of games that were found-->
        <li v-for="(game) in games" :key="game.appid">
            <button ref="game_ref" @click="selectNewGame($event, game)">{{game.public_name}}</button>  <!--    Each item on the list will send its-->
        </li>                                                                                          <!-- game data to the select function when clicked-->
    </div>
    <div class="not-found-list">
        <li v-for="(game) in supported_games" :key="game.app_id" >
            <button ref="supported_game_ref" v-if="!games[game.app_id]" @click="selectSupportedGame($event, game)">{{  game.public_name }}</button>
        </li>  <!-- Similar to the buttons in the found list, however these get hidden if the game they're for gets found in a scan-->
    </div>
    <div class="options-bottom">
        <!--<button class="run-button">Run</button> I don't see how the run button could be feasible,
            we would have to run games either through steam or wine or lutris or heroic and there's just too many potential launchers imo -->
        <button class="deploy-button" @click="sendDeployMods()">Deploy</button>
    </div>
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
        background-color: rgba(255,255,255,7%);
        border-top-left-radius: 5px;
        border-top-right-radius: 5px;
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

    .not-found-list {
        flex: 1 0 auto;
        background-color: rgba(255,255,255,7%);
        border-bottom-left-radius: 5px;
        border-bottom-right-radius: 5px;
        overflow: hidden;
        overflow-y: scroll;
        li {
            list-style: none;
            width: 100%;
            display: flex;
            flex-direction: row;
            &:last-child {
                border-bottom: none;
            }
            button {
                text-align: center;
                display: block;
                width: 100%;
                margin: 5px;
                background-color: rgba(255,255,255,0%);
                color: rgb(58, 58, 58);
                &:hover {
                    background-color: rgba(255,255,255,4%);
                }
                &.active {
                    background-color: rgba(255,255,255,9%);
                    color: rgb(10, 10, 10) 
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
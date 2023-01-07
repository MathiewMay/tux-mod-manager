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
    setup() {
        const games = reactive({})
        return {games}
    },
    methods: {
        selectNewGame(e, gameEntry){
            const gameButton = e.target 
            const buttonList = this.$refs.game_ref
            buttonList.forEach(elem => {
                elem.classList.remove("active");
            })
            const secondButtonList = this.$refs.supported_game_ref
            secondButtonList.forEach(elem => {
                elem.classList.remove("active");
            })
            gameButton.classList.add("active");
            this.$emit('on-game-selected', gameEntry)
        },

        selectSupportedGame(e){  // Same function as above, only for the games that aren't scanned yet so the selectedgame variable isn't updated with a non existant game
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
        },

        async scanGames() {
            await invoke('scan_games', { supportedGames: supported_games }).then((entrys) => {
                entrys.forEach(element => {
                    let game = JSON.parse(element)
                    // if(this.supported_games[game.appid]){
                    this.games[game.appid] = game
                    // }
                })
            })
            this.$emit('on-scan-games')
        }, 
    
        sendDeployMods() {
            this.$emit('deploy-mods')
        },
    }
}
</script>

<template>
<div class="side-bar">
    <button class="scan-games-button" @click="scanGames()">Scan games</button>  <!-- activates the scanGames method above-->
    <div class="game-list">
        <li v-for="(game) in games" :key="game.appid">
            <button ref="game_ref" @click="selectNewGame($event, game)">{{game.public_name}}</button>
        </li>
    </div>
    <div class="not-found-list">
        <li v-for="(game) in supported_games" v-if="!Object.keys(games).some(appid => appid === key)" :key="game.app_id" >  <!-- TODO, fix this v-if="!games.some(scanned_game => scanned_game.appid === game.app_id)" so the list item disappears if it gets found in a scan-->
            <button ref="supported_game_ref" @click="selectSupportedGame($event)">{{ game.public_name }}</button>
        </li>
    </div>
    <div class="options-bottom">
        <button class="run-button">Run</button>
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
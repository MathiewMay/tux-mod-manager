<script>
import { ref } from '@vue/reactivity'

import ModManager from './ModManager.vue'
import ModInstaller from './ModInstaller.vue'
import ModDownloader from './ModDownloader.vue'
import { invoke } from '@tauri-apps/api'

export default {
    data() {
        return {
            tab: 2
        };
    },
    props: ['selected_game', 'games', 'supported_game'],
    components: {
      ModManager,
      ModInstaller,
      ModDownloader,
    },
    setup() {
        const initialState = {}
        const mods = ref({...initialState})
        function resetMods(){
            mods.value = {...initialState}
        }
        return {mods, resetMods}
    },
    methods: {
        async newGameSelected() {
            setTimeout(() => // after a new game is selected, wait 1 millisecond, then run refreshModList in ModManager.vue
                {            // refreshModList accesses Rust backend get-mods function to update the mod data for mod manager
                    this.$refs.mod_manager.refreshModList();
                }, 
                1,
            );
        },
        async refreshModList() {  
            this.$refs.mod_manager.refreshModList();
        },
        async deployMods() {
            this.$refs.mod_manager.deployMods();
        },
        async scanGame(supported_game) {  // Scan for a single game, intended to speed up the slow backup search but it will attempt the faster steamlocate search first
            await invoke('scan_for_single_game', { game: supported_game }).then((entries) => {  // calls the rust backend scan_for_single_game function, located in mod_manager.rs
                entries.forEach(element => {
                    let game = JSON.parse(element)
                    this.games[game.appid] = game  // adds each game found to the list of games, identified by its appid
                })
            })
        },
    }
}
</script>

<template>
<div class="main-panel">
    <div class="tab-selector">  <!-- clicking on the different tabs updates the tab variable for the tabs div below-->
        <button v-if="selected_game.public_name" @click="tab = 0" :class="{active: tab == 0}">Load Order</button>  <!-- these buttons hide when there's no found selected game-->
        <button v-if="selected_game.public_name" @click="tab = 1" :class="{active: tab == 1}">Downloads</button>
        <button @click="tab = 2" :class="{active: tab == 2}">Settings</button>  
    </div>

    <div class="tabs">
    
        <div v-if="selected_game.public_name" class="tab load-order" :class="{visible: tab == 0}"> <!-- Load Order-->
            <div class="mod-order">
                <ModManager ref="mod_manager" :selected_game="selected_game"></ModManager>
            </div>
            <div class="mod-order">
                <ModInstaller @on-mod-installed="refreshModList()" :selected_game="selected_game"/>
            </div>
        </div>

        <div v-if="selected_game.public_name" class="tab" :class="{visible: tab == 1}">  <!-- Downloads -->
            <ModDownloader :selected_game="selected_game"></ModDownloader>
        </div>

        <div class="tab" :class="{visible: tab == 2}">  <!-- Settings -->
            <button class="scan_button" ref="game_scan" v-if="supported_game.public_name" @click="scanGame(supported_game)">Scan Game</button> <!-- The scan game button only shows when a supported_game is selected-->
        </div>                                                                                                             <!-- send that supported_game to the scan function-->

    </div>
</div>
</template>

<style lang="scss" scoped>
.main-panel {
    flex: 1 1 auto;
    overflow: auto;
    height: 100vh;
    .tab-selector {
        padding-top: 5px;
        padding-inline: 5px;
        position: sticky;
        z-index: 1;
        top: 0;
        border-bottom: 1px solid rgba(255,255,255,8%);
        background: #121212;
        button {
            border-bottom-right-radius: 0;
            border-bottom-left-radius: 0;
            margin: 0;
            margin-right: 5px;
            padding: 5px;
            font-size: 1.02em;
            &:last-child {
                margin-right: 0;
            }
        }
        button.active {
            background-color: rgba(255,255,255,14%);
        }
    }
    .tabs {
        .tab {
            overflow: auto;
            display: none;
            &.visible {
                display: block;
            }
            .scan_button {
                position: fixed;
                bottom: 0; right: 0;
                margin: 10px;
                height: 30px;
                padding: 5px;
                padding-inline: 15px;
            }
        }
        .load-order {
            .mod-order {
                padding: 0;
                .mod-list {
                    list-style: none;
                    border-bottom: 1px solid rgba(255,255,255,8%);
                    &:last-child {
                        border-bottom: none;
                    }
                }
            }
        }
    }
}
</style>
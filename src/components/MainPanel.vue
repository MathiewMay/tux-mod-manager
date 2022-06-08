<script>
import { ref } from '@vue/reactivity'
import { invoke } from '@tauri-apps/api/tauri'

import ModManager from './ModManager.vue'
import ModInstaller from './ModInstaller.vue'
import ModDownloader from './ModDownloader.vue'

export default {
  data() {
    return {
      tab: 0
    };
  },
  props: ['selected_game'],
  components: {
    ModManager,
    ModInstaller,
    ModDownloader
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
    // async refreshModList(){
    //   this.resetMods()
    //   const modsEntrys = await invoke('get_mods', {game: this.selected_game})
    //   modsEntrys.forEach(modEntry => {
    //     const modJson = JSON.parse(modEntry)
    //     this.mods[modJson.name] = modJson
    //   })
    // },

    // async deployMods(){
    //   console.log("deploying mods");
    //   if(this.$refs.mod_ref != undefined){
    //     const modList = this.$refs.mod_ref
    //     let enabledMod = []
    //     for(var i=0; i<modList.length; i++){
    //       const mod_ref = modList[i]
    //       if(mod_ref.$refs.mod_enabled.checked)
    //         enabledMod.push(mod_ref.mod)
    //     }
    //     invoke('deploy', { mods: enabledMod, game: this.selected_game })
    //   }
    // }
    async newGameSelected(game) {
      setTimeout(() => { 
        if (this.$refs.mod_manager != undefined) {
          this.$refs.mod_manager.refreshModList();
        }
      }, 1);
    },
    async refreshModList() {
      this.$refs.mod_manager.refreshModList();
    },
    async deployMods() {
      if (this.$refs.mod_manager != undefined) {
        this.$refs.mod_manager.deployMods();
      }
    }
  }
}
</script>

<template>
<div class="main-panel">
  <div class="tab-selector">
    <button v-if="selected_game.public_name" @click="tab = 0" :class="{active: tab == 0}">Load Order</button>
    <button v-if="selected_game.public_name" @click="tab = 1" :class="{active: tab == 1}">Downloads</button>
    <button @click="tab = 2" :class="{active: tab == 2}">Settings</button>
  </div>
  <div class="tabs">
    <div v-if="selected_game.public_name" class="tab load-order" :class="{visible: tab == 0}">
      <div class="mod-order">
        <ModManager ref="mod_manager" :selected_game="selected_game"></ModManager>
      </div>
      <div class="mod-order">
        <ModInstaller @on-mod-installed="refreshModList()" :selected_game="selected_game"/>
      </div>
    </div>
    <div v-if="selected_game.public_name" class="tab" :class="{visible: tab == 1}">
      <ModDownloader :selected_game="selected_game"></ModDownloader>
    </div>
    <div class="tab" :class="{visible: tab == 2}">

    </div>
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
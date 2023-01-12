<template>
    <div @deploy-mods="deployMods()" class="manager-mod-order">
        <li class="manager-mod-list" v-for="(mod) in mods" :key="mod">
            <Mod ref="mod_ref" :selected_game="selected_game" :mod="mod" :installing="false"/>
        </li>
    </div>
</template>

<script>
import { ref } from '@vue/reactivity'
import { dialog } from '@tauri-apps/api'
import { invoke } from '@tauri-apps/api/tauri'

import Mod from './Mod.vue'

export default {
    props: ['selected_game'],
    components: {
        Mod,
    },
    setup() {
        const initialState = {}
        const mods = ref({...initialState})
        function resetMods() {
            mods.value = {...initialState}
        }
        return {mods, resetMods}
    },
    methods: {
        async refreshModList(){
            // console.log("Mod List refreshed");
            this.resetMods()
            const modsEntrys = await invoke('get_mods', {game: this.selected_game})
            modsEntrys.forEach(modEntry => {
                const modJson = JSON.parse(modEntry)
                this.mods[modJson.name] = modJson
            })
        },
    
        async deployMods(){
            // console.log("hi");
            if(this.$refs.mod_ref != undefined){
                const modList = this.$refs.mod_ref
                let enabledMod = []
                for(var i=0; i<modList.length; i++){
                    const mod_ref = modList[i]
                    if(mod_ref.$refs.mod_enabled.checked)
                        enabledMod.push(mod_ref.mod)
                }
                invoke('deploy', { mods: enabledMod, game: this.selected_game })
            }
        }
    }
}
</script>

<style lang="scss" scoped>
  .manager-mod-order {
      width: 100%;
      overflow-y: scroll;
      .manager-mod-list { 
          list-style: none;
          border-bottom: 1px solid rgba(255,255,255,8%);
          &:last-child {
              border-bottom: none;
          }
      }
  }
</style>
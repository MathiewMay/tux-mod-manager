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
    function resetMods(){
      mods.value = {...initialState}
    }
    return {mods, resetMods} 
  },
  methods: {
    async installMod() {
      dialog.open({filters: [{ extensions: ['zip', 'rar', '7z'], name: "Archives" }]}).then((file) => {
        const fileFullName = file.split('/')[file.split('/').length-1]
        const fileName = fileFullName.split('.')[0]
        this.mods[fileName] = {name: fileName}
        invoke('uncompress', { filePath: file, fileName: fileName, game: this.selected_game}).then(()=>{
          this.$emit('on-mod-installed')
          delete this.mods[fileName]
        })
      })
    }
  }
}
</script>

<template>
<div class="install-mod-order">
  <button class="install-button" @click="installMod()">Install Mods</button>
  <li class="install-mod-list" v-for="(mod) in mods" :key="mod">
    <Mod :selected_game="selected_game" :mod="mod" :installing="true"/>
  </li>
</div>
</template>

<style lang="scss" scoped>
.install-mod-order {
  width: 100%;
  overflow-y: scroll;
  .install-mod-list { 
    list-style: none;
    border-top: 1px solid rgba(255,255,255,8%);
  }
  .install-button {
    position: fixed;
    bottom: 0; right: 0;
    margin: 10px;
    height: 30px;
    padding: 5px;
    padding-inline: 15px;
  }
}
</style>
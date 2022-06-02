<script>
import { invoke } from '@tauri-apps/api/tauri'
import { dialog } from '@tauri-apps/api'

export default {
  props: ['mod', 'selected_game'],
  methods: {
    async removeMod() {
      dialog.ask("You are about to delete \n\""+this.mod.name+"\" \nare you sure you want to proceed?").then((proceed) => {
        if(proceed){
          invoke('remove_mod', { modStruct: this.mod })
          this.$parent.refreshModList()
        }
      })
    }
  }
}
</script>

<template>
<div class="mod">
  <input ref="mod_enabled" class="mod-enabled" type="checkbox">
  <p class="mod-name">{{ mod.name }}</p>
  <div class="mod-options">
    <button class="remove-button" @click="removeMod()">Remove</button>
  </div>
</div>
</template>

<style scoped>
.mod {
  display: flex;
  flex-direction:row;
  justify-content: left;
}
.mod-enabled {
  float: left;
  width: 30px;
  height: 30px;
}
.mod-name {
  color: white;
  float: left;
  width: 100%;
  white-space: nowrap;
  overflow: hidden;
  margin: 0px;
  margin-top: 8px;
  margin-left: 10px;
}
.mod-options {
  float: right;
}
.mod-options button {
  margin-top: 3px;
}
.remove-button {
  width: 89px;
  height: 30px;
}
</style>
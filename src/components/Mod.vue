<script>
import { invoke } from '@tauri-apps/api/tauri'
import { dialog } from '@tauri-apps/api'

export default {
  props: ['mod', 'selected_game', 'installing'],
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
  <input v-if="installing != true" ref="mod_enabled" class="mod-enabled" type="checkbox">
  <i v-else class="fa fa-duotone fa-arrows-rotate fa-spin"></i>
  <p class="mod-name">{{ mod.name }}</p>
  <div class="mod-options">
    <button v-if="installing != true" class="end-button" @click="removeMod()">Remove</button>
    <button v-else class="end-button" @click="()=>{}">Cancel</button>
  </div>
</div>
</template>

<style scoped>
.fa {
  color: white;
  margin-top: 5px;
  margin-left: 5px;
  font-size: x-large;
}
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
  margin: 5px;
}
.end-button {
  width: 90px;
  height: 30px;
}
</style>
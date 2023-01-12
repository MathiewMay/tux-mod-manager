<script>
import { invoke } from '@tauri-apps/api/tauri'
import { dialog } from '@tauri-apps/api'

export default {
    props: ['mod', 'selected_game', 'installing'],
    methods: {
        async removeMod() {
            dialog.ask("You are about to delete \n\""+this.mod.name+"\" \nare you sure you want to proceed?").then((proceed) => {
                if(proceed){
                    invoke('remove_mod', { modStruct: this.mod, game: this.selected_game })
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

<style scoped lang="scss">
    .fa {
        color: white;
        font-size: x-large;
        margin-right: 10px;
        width: 30px; line-height: 30px;
        text-align: center;
    }
    .mod {
        display: flex;
        flex-direction:row;
        justify-content: left;
        padding: 5px;
    }
    .mod-enabled {
        width: 30px;
        height: 30px;
        margin: 0;
        margin-right: 10px;
    }
    .mod-name {
        color: white;
        float: left;
        width: 100%;
        white-space: nowrap;
        overflow: hidden;
        margin: 0px;
        line-height: 30px;
        height: 30px;
    }
    .mod-options button {
        margin: 0;
        height: 30px;
    }
</style>
<template>
  <div class="mod-downloader">
    <Download v-for="(download) in downloads" ref="download" :key="download" :filename="download.filename" :install_status="download.install_status" :progress="download.progress"/>
    <div class="url-downloader">
      <input type="url" name="url" id="url" ref="url">
      <button @click="download()">Download</button>
    </div>
  </div>
</template>

<script>
import { ref } from '@vue/reactivity'
import { event, dialog } from '@tauri-apps/api'
import { invoke } from '@tauri-apps/api/tauri'

import Download from './Download.vue'
export default {
    props: ['selected_game'],
    components: {
        Download
    },
    setup() {
        const downloads = ref([]);
    
        event.listen("download-started", event => {
            // console.log("Download Started: " + event.payload.filename)
            let install_status = 1;
            if (event.payload.filesize != null) {
                install_status = 0;
            }
            downloads.value.push(
                {
                    filename: event.payload.filename,
                    install_status: install_status,
                    progress: 0
                }
            );
        })
        event.listen("download-progress", event => {
            // console.log("Download Progress: " + event.payload.filename + " " + event.payload.current + "/" + event.payload.filesize);
            downloads.value.forEach(element => {
                if (element.filename == event.payload.filename){
                    if (event.payload.filesize != null) {
                        var dif = event.payload.current / event.payload.filesize * 100;
                        // console.log("dif: " + dif);
                        if (dif - element.progress > .1) {
                            let temp = Math.floor(dif * 10) / 10;
                            // console.log("temp: " + temp);
                            element.progress = temp;
                        }
                    } else {
                        var temp = event.payload.current;
                        if ((temp / element.progress) > 1.01) {
                            element.progress = temp;
                        }
                    }
                }
            });
        });
        event.listen("download-finished", event => {
            // console.log("Download Finished: " + event.payload.filename);
            downloads.value.forEach(element => {
                if (element.filename == event.payload.filename){
                    element.install_status = 2;
                }
            });
        });
        event.listen("already-downloaded", event => {
            dialog.message("You have already downloaded this file:\n'" + event.payload + "'");
        })
        return { downloads }
    },
    methods: {
        async download() {
            console.log(this.$refs.url.value);
            invoke('download', { url: this.$refs.url.value, game: this.selected_game });
        },
        test() {
            console.log(1);
        }
    }
}
</script>

<style lang="scss" scoped>
.mod-downloader {
    width: 100%;
    color: #fff;
    overflow: auto;
    // padding: 5px;
    position: relative;
    .url-downloader {
        background: #121212;
        border: 1px solid rgba(255,255,255,8%);
        border-radius: 5px;
        position: fixed;
        bottom: 0;
        right: 0;
        margin: 5px;
        padding: 5px;
        button {
            margin: 0;
            height: 30px;
        }
        input[type=url] {
            user-select: text;
            -webkit-user-select: text;
            -webkit-user-drag: auto;
            height: 30px;
            line-height: 30px;
            border-radius: 5px;
            appearance: none;
            margin: 0;
            padding-inline: 5px;
            border: 2px solid rgba(255,255,255,8%);
            background: rgba(255,255,255,7%);
            color: #fff;
            margin-right: 5px;
            &:focus {
                border: 2px solid rgba(255,255,255,14%);
                background-color: rgba(255,255,255,12%);
            }
            &:hover {
                background-color: rgba(255,255,255,12%);
            }
        }
    }
}
</style>
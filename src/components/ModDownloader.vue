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
import { event } from '@tauri-apps/api'
import { invoke } from '@tauri-apps/api/tauri'

import Download from './Download.vue'
export default {
  props: ['selected_game'],
  components: {
    Download
  },
  data() {
    return {
      downloads: [
        {
          filename: "mod_that_is_downloading-v1.18.2.7z",
          install_status: 0,
          progress: 100
        },
        {
          filename: "AnotherDownload-CBBE-Bodyslide.rar",
          install_status: 0,
          progress: 90
        },
        {
          filename: "HelloThere AlternateStartAddon.zip",
          install_status: 0,
          progress: 65
        },
        {
          filename: "OpenDownloadSimulator-3.32.7z",
          install_status: 0,
          progress: 69
        },
        {
          filename: "a_very_long_modname_for_testing-Alpha-3b.rar",
          install_status: 0,
          progress: 89
        },
        {
          filename: "ImWatchingDougDoug-right-now-twitch.tv",
          install_status: 0,
          progress: 99
        },
        {
          filename: "DownloadTest-v2.7z",
          install_status: 0,
          progress: 1
        },
        {
          filename: "Downloaded Mod V.12",
          install_status: 1,
          progress: 100
        },
        {
          filename: "Success_in_download-3.3",
          install_status: 1,
          progress: 100
        },
        {
          filename: "File_is_here.zip",
          install_status: 1,
          progress: 100
        },
        {
          filename: "installed-mod-30",
          install_status: 2,
          progress: 100
        },
        {
          filename: "hey-this-is-installed.rar",
          install_status: 2,
          progress: 100
        },
        {
          filename: "yo-done.zip",
          install_status: 2,
          progress: 100
        }
      ]
    }
  },
  setup() {
    event.listen("download-started", event => {
      console.log("Download Started: " + event.payload)
    })
    event.listen("download-progress", event => {
      console.log("Download Progress: " + event.payload.filename + " " + event.payload.current + "/" + event.payload.filesize);
    });
    event.listen("download-finished", event => {
      console.log("Download Finished: " + event.payload.filename);
    });
  },
  methods: {
    async download() {
      console.log(this.$refs.url.value);
      invoke('download', { url: this.$refs.url.value, game: this.selected_game });
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
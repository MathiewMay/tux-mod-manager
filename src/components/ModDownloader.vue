<template>
  <div class="mod-downloader">
    <Download v-for="(download) in downloads" :key="download" :filename="download.filename" :install_status="download.install_status" :progress="download.progress"/>
  </div>
</template>

<script>
import { ref } from '@vue/reactivity'
import { dialog } from '@tauri-apps/api'
import { invoke } from '@tauri-apps/api/tauri'

import Download from './Download.vue'
export default {
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
  }
}
</script>

<style lang="scss" scoped>
.mod-downloader {
  width: 100%;
  color: #fff;
  overflow: auto;
  // padding: 5px;
}
</style>
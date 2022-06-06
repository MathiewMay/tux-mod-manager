<template>
  <div class="mod-downloader">
    <table class="downloads">
      <Download v-for="(download) in downloads" :key="download" :filename="download.filename" :install_status="download.install_status" :progress="download.progress"/>
    </table>
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
          filename: "mod_that_is_downloading-v1",
          install_status: 0,
          progress: 50
        },
        {
          filename: "Downloaded Mod V.12",
          install_status: 1,
          progress: 100
        },
        {
          filename: "installed-mod-30",
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
  padding: 5px;
}
</style>

<style lang="scss">
td, table {
  border: 1px solid rgba(255,255,255,7%);
}
table {
    width: 100%;
    border-collapse: collapse;
    tr {
      border-collapse: collapse;
      width: 100%;
      td {
        border-collapse: collapse;
        padding: 5px;
        // &:first-child {
        //   flex: 1 0 auto;
        // }
      }
    }
  }
</style>
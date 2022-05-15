<script>

import SideBar from './components/SideBar.vue'
import ModManager from './components/ModManager.vue'

export default {
  components: {
    SideBar,
    ModManager
  }
}

</script>

<template>
  <SideBar />
  <ModManager />
</template>

<style>
*{
  -webkit-user-select: none;
  user-select: none;
}
body {
  margin: 0px;
  background-color: #121212;
}
button {
  border: none;
  border-radius: 5px;
  background-color: rgba(255,255,255,9%);
  color: rgba(255,255,255,100%);
}
button:hover {
  cursor: pointer;
  background-color: rgba(255,255,255,12%);
}
</style>

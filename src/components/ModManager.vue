<script>
import Mod from './Mod.vue'

export default {
  components: {
    Mod,
  }
}
</script>

<template>
<div class="mod-manager">
  <div class="mod-order">
    <li class="mod-list">
      <Mod />
    </li>
  </div>
  <div class="separator-bottom"></div>
</div>
</template>

<style scoped>
.mod-manager {
  display: inline-flex;
  width: 865px;
}
.mod-order {
  width: 100%;
  max-height: 500px;
  overflow: hidden;
  overflow-y: scroll;
}
.mod-list { 
  margin: 5px;
  list-style: none;
}
.separator-bottom{
    color: black;
    background-color: rgba(255,255,255,8%);
    width: 864px;
    height: 1px;
    position: absolute;
    margin-left: 1px;
    top: 500px;
}
</style>
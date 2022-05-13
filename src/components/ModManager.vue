<script>
import ModList from './ModList.vue'

export default {
  components: {
    ModList,
  }
}
</script>

<template>
<div class="mod-manager">
  <div class="mod-order">
    <ModList />
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
<script>
export default {
  props: ['mod_name'],
}
</script>

<template>
<div class="mod">
  <input class="mod-enabled" type="checkbox">
  <p class="mod-name">{{ mod_name }}</p>
  <div class="mod-options">
    <button class="remove-button">Remove</button>
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
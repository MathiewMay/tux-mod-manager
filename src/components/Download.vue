<template>
  <tr>
    <td>
      {{ filename }}
    </td>
    <td>
      <div v-if="installed">
        Hi
      </div>
      <div v-if="!installed">
        Bye
      </div>
    </td>
  </tr>
</template>

<script>
export default {
    props: ['filename', 'installed']
}
</script>

<style lang="scss" scoped>

</style>
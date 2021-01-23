<template>
  <div class="page-wrapper">
    <h3>Library</h3>
    <span class="link" @click="browse">Install from Computer</span>
  </div>
</template>

<script>
import { open } from "tauri/api/dialog"
import { emit } from "tauri/api/event"

export default {
  name: "Index",
  methods: {
    browse(){
      open().then((file) => {
        emit("sinix-install", file)
        console.log(file)
      }).catch((err) => {
        console.log(err)
      })
    }
  }
}
</script>

<style scoped>
.page-wrapper {
  padding: 1rem;
}
</style>

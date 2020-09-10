<template>
  <div class="sidebar-wrapper">
    <div class="item">
      Explore
    </div>
    <div class="item">
      Installed
    </div>
    <div class="item" @click="browse">
      Browse
    </div>
  </div>
</template>

<script>
import { open } from "tauri/api/dialog"

export default {
  methods: {
    browse(){
      open({
        filter: "*.dext"
      }).then((file) => {
        console.log(file)
      }).catch((err) => {
        console.log(err)
      })
    }
  }
}
</script>

<style scoped>
.sidebar-wrapper {
  background: var(--bg-color);
  border: 1px solid var(--border-color);

  @apply p-3 h-full;
}

.item {
  @apply uppercase text-sm;
  @apply cursor-pointer;
  @apply w-full px-4 py-1 rounded-full;
}

.item:hover {
  background-color: #dc143c14;
}
</style>

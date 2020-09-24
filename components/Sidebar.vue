<template>
  <div class="sidebar-wrapper">
    <nuxt-link to="/explore">
      <div class="item">
        Explore
      </div>
    </nuxt-link>
    <nuxt-link to="/installed">
      <div class="item">
        Installed
      </div>
    </nuxt-link>
    <div class="item" @click="browse">
      Browse
    </div>
  </div>
</template>

<script>
import { open } from "tauri/api/dialog"
import { emit, listen } from "tauri/api/event"

export default {
  mounted(){
    listen("sinix-install-response", (payload) => {
      console.log(payload)
    })
  },
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
.sidebar-wrapper {
  background: var(--bg-color);
  border-right: 0.1px solid var(--border-color);

  @apply p-3 h-full;
}

.item {
  @apply uppercase text-sm;
  @apply cursor-pointer;
  @apply w-full px-4 py-1 rounded-full;
}

.item:hover {
  color: #DC143C;
}
</style>

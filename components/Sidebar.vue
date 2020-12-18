<template>
  <div class="sidebar-wrapper">
    <nuxt-link to="/">
      <div class="item">
        Library
      </div>
    </nuxt-link>
    <nuxt-link to="/explore">
      <div class="item">
        Explore
      </div>
    </nuxt-link>
    <nuxt-link to="/collections">
      <div class="item">
        Collections
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
  padding: 1rem;
}

.item {
  width: 100%;
  cursor: pointer;
  font-size: 0.875rem;
  border-radius: 100%;
  padding: 0.25rem 1rem;
  text-transform: uppercase;
}

.item:hover {
  color: #DC143C;
}
</style>

<template>
  <div class="sidebar-wrapper">
    <nuxt-link to="/">
      <div class="item">
        <span>Library</span>
      </div>
    </nuxt-link>
    <nuxt-link to="/explore">
      <div class="item">
        <span>Explore</span>
      </div>
    </nuxt-link>
    <nuxt-link to="/collections">
      <div class="item">
        <span>Collections</span>
      </div>
    </nuxt-link>
    <div class="item" @click="browse">
      <span>Browse</span>
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
.item {
  width: 100%;
  padding: 1rem;
  color: #A1A0A0;
  cursor: pointer;
  font-size: 0.9rem;
  padding-left: 1.2rem;
  font-family: 'Roboto Black';
  border-bottom: 0.1px solid var(--border-color);
}

.item:hover {
  color: #DC143C;
  background: #F5F5F5;
}
</style>

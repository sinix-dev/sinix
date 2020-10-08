<template>
  <div class="sidebar-wrapper">
    <button type="button" class="btn">
      <nuxt-link to="/">
        <div class="item">
          Home
        </div>
      </nuxt-link>
    </button>
    <button type="button" class="btn">
      <nuxt-link to="/explore">
        <div class="item">
          Explore
        </div>
      </nuxt-link>
    </button>
    <button type="button" class="btn">
      <nuxt-link to="/installed">
        <div class="item">
          Installed
        </div>
      </nuxt-link>
    </button>
    <button type="button" class="btn">
      <div class="item" @click="browse">
      Browse
      </div>
    </button>
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
}

.item {
  width: 100%;
  cursor: pointer;
  font-size: 0.875rem;
  border-radius: 100%;
  padding: 0.25rem 1rem;
  text-transform: uppercase;
  color: #A1A0A0;
;
}

.item:hover {
  color: aqua;
}

button {
  background: rgba(21, 36, 56, 0.95);
  width: 100%;
}

button:hover{
  background: #223146;
}
</style>

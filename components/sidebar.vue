<template>
  <div class="sidebar-wrapper">
    <nuxt-link to="/">
      <div class="item">
        <client-only>
          <feather class="icon" type="heart" />
        </client-only>
        <span>Library</span>
      </div>
    </nuxt-link>
    <nuxt-link to="/explore">
      <div class="item">
        <client-only>
          <feather class="icon" type="compass" />
        </client-only>
        <span>Explore</span>
      </div>
    </nuxt-link>
    <nuxt-link to="/collections">
      <div class="item">
        <client-only>
          <feather class="icon" type="layers" />
        </client-only>
        <span>Collections</span>
      </div>
    </nuxt-link>
    <nuxt-link to="/settings">
      <div class="item">
        <client-only>
          <feather class="icon" type="settings" />
        </client-only>
        <span>Settings</span>
      </div>
    </nuxt-link>
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

<style>
.icon {
  width: 17px;
  height: 17px;
  margin-right: 5px;
  color: #DC143C;
}
</style>

<style scoped>
.sidebar-wrapper {
  display: flex;
  padding: 10px;
  flex-direction: column;
  height: calc(100% - 20px);
}

.item {
  padding: 8px;
  display: flex;
  cursor: pointer;
  font-size: 14px;
  border-radius: 5px;
  align-items: center;
}

.item:hover {
  background: #FDD9D9;
}
</style>

<template>
  <div class="sidebar-wrapper">
    <div class="search-bar">
      <client-only>
        <div class="search-icon-wrapper">
          <feather class="search-icon" type="search" />
        </div>
      </client-only>
      <div>
        <input class="search-input" type="text" placeholder="Search">
      </div>
    </div>
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
    <nuxt-link to="/logs">
      <div class="item">
        <client-only>
          <feather class="icon" type="radio" />
        </client-only>
        <span>Logs</span>
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
  color: var(--color-primary);
}

.search-icon {
  width: 17px;
  height: 17px;
  vertical-align: middle;
}
</style>

<style scoped>
.search-bar {
  display: flex;
  padding: 6px 8px;
  margin-bottom: 8px;
  border-radius: 3px;
  background: var(--bg-extra);
}

.search-input {
  border: none;
  font-size: 14px;
  background: transparent;
}

.search-icon-wrapper {
  margin-right: 5px;
}

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
  background: var(--color-highlight);
}
</style>

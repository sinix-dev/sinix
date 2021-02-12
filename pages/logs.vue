<template>
  <div class="page-wrapper">
    <h3>Logs</h3>
    <div class="logs-wrapper">
      <div v-for="log in logs" :key="log.payload" class="udp-log">
        <span class="type">{{ log.type }}:</span>
        <span>{{ log.payload }}</span>
      </div>
    </div>
  </div>
</template>

<script>
import { listen } from "tauri/api/event"

export default {
  data(){
    return {
      logs: []
    }
  },
  mounted(){
    listen("udp-event", (response) => {
      console.log(response)
      this.logs.unshift(response)
    })
  }
}
</script>

<style scoped>
.page-wrapper {
  padding: 1rem;
}

.logs-wrapper {
  height: 600px;
  overflow-y: scroll;
}

.type {
  font-weight: bold;
  color: green;
}
</style>

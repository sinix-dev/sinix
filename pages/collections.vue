<template>
  <div class="page-wrapper">
    <h3>Collection</h3>
    <div v-for="game in games" :key="game.slug">
      <span class="link" @click="open(game.slug)">
        {{ game.name }} v{{ game.version }}
      </span>
    </div>
  </div>
</template>

<script>
import { emit, listen } from "tauri/api/event"

export default {
  data(){
    return {
      games: []
    }
  },
  mounted(){
    listen("game-item", (response) => {
      console.log(response)
      this.games.push(response.payload)
    })
    emit("game-list")
  },
  methods: {
    open(game_id){
      emit("game-webview", game_id)
    }
  }
}
</script>

<style scoped>
.page-wrapper {
  padding: 1rem;
}
</style>

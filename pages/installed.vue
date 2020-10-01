<template>
  <div class="page-wrapper">
    <h1>Installed Games</h1>
    <div v-for="game in games" :key="game.name">
      <a class="link" :href="`http://127.0.0.1:41432/${game.name}/index.html`">{{ game.name }}</a>
    </div>
  </div>
</template>

<script>
import * as fs from "tauri/api/fs"

export default {
  data(){
    return {
      games: []
    }
  },
  created(){
    if (process.client){
      fs.readDir("/home/sanket143/.sinix/games")
        .then((files) => {
          console.log(files)
          this.games = files
        })
        .catch((err) => {
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

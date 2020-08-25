<template>
  <div class="p-3">
    <h1>Main</h1>
    <input type="text" v-model="msg">
    <button @click="send">Send</button>
  </div>
</template>

<script>
import { open } from "tauri/api/dialog"

export default {
  data(){
    return {
      msg: "",
      file: ""
    }
  },
  methods: {
    async send(){
      const file = await open()
      console.log(file)
    },
    fileChange(){
      console.log(document.getElementById("myFile").files)
    }
  }
}
</script>

<style>
button, input {
  border: 1px solid grey;
}

</style>

<template>
  <div class="p-3">
    <h1>Main</h1>
    <input v-model="msg" type="text">
    <button @click="send">
      Send
    </button>
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
    send(){
      open({
        filter: "*.dext"
      }).then((file) => {
        console.log(file)
      }).catch((err) => {
        console.log(err)
      })
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

<template>
  <div class="p-3">
    <h1>Main</h1>
    <input type="text" v-model="msg">
    <button @click="send">Send</button>
  </div>
</template>

<script>
import { emit, listen } from "tauri/api/event"

export default {
  data(){
    return {
      msg: ""
    }
  },
  methods: {
    send(){
      console.log("event occured", this.msg)
      emit("sinix-ping", this.msg)
      listen("sinix-event", (payload) => {
        console.log("EVENT", payload)
      })
    }
  }
}
</script>

<style>
button, input {
  border: 1px solid grey;
}

</style>

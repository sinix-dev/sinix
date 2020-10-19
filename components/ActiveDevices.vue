<template>
  <div class="container">
    <li v-for="(item, index) in devices" v-bind:key="index"><Device v-bind:title="item" /></li>
  </div>
</template>

<script>
export default {
  data(){
    return{
      devices: ["Allen", "Rajvit", "Anirudh", "Lanket", "M10", "Paurabh", "Lanket", "M10", "Paurabh"]
    }
  }
}
</script>

<style scoped>
@import "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css";

.container {
    display: absolute;
    display: grid;
    align-content: flex-start;
    padding: 10px;
    height: 100%;
    width: 100%;
}

.fa-user{
    padding: 2px;
}
li {
    display: block;
    list-style: none;
}
hr{
  margin: 2px;
}
h4{
  margin: 2px;
}

</style>

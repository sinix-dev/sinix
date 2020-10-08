<template>
  <div class="">
    <li v-for="(item, index) in devices" v-bind:key="index"><Device v-bind:title="item" /></li>
  </div>
</template>

<script>
export default {
  data(){
    return{
      devices: ["Shalom", "Rajvit", "Anirudh", "Sanket", "M10", "Saurabh"]
    }
  }
}
</script>

<style scoped>
@import "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css";

.fa-user{
    padding: 2px;
}
li {
    list-style: none;
}
hr{
  margin: 2px;
}
h4{
  margin: 2px;
}

</style>

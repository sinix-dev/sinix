<template>
  <div>
    <div class="page-wrapper">
      <h1>Home</h1>
      <form @submit.prevent="urlHandler">
        <input v-model="url" type="text" placeholder="Search your game">
        <button v-on:click="urlHandler()">Search</button>
      </form>
      <br>
      <!--src attribute is hard coded-->
      <iframe v-if: src ="https://games.cdn.famobi.com/html5games/t/table-tennis-world-tour/v240/?fg_domain=play.famobi.com&fg_aid=A1000-1&fg_uid=8c8e5fc6-7e38-47ab-96eb-150276e79ef8&fg_pid=4638e320-4444-4514-81c4-d80a8c662371&fg_beat=736&original_ref=https%3A%2F%2Fgames.cdn.famobi.com%2F" height="550px" width="100%" title="Iframe Example"></iframe>
    </div>
  </div>
</template>

<script>
export default {
  name: "Index",
  components: {
  },
  // Currently I want to use this url attribute in the src of iframe but the im not sure how to pass the data to it.
  data(){
    return {
      url: ""
    }
  },
  methods: {
    urlHandler(){
    }
  }
}

</script>

<style scoped>
.page-wrapper {
  padding: 1rem;
}

button{
  width: 200px;
  border:20;
  padding: 20px;
  font-size: 1.3em;
  background-color: aqua;
  color: #687F7F;
}

button:hover{
  opacity: 0.50;
}

input {
  width: calc(50% - 50px);
  border: 0;
  padding: 20px;
  font-size: 1.3em;
  background-color: #323333;
  color: #687F7F;
}
</style>

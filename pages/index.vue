<template>
  <div>
    <div class="page-wrapper">
      <h1>Home</h1>
      <form class="searchbar">
        <div class="searchbar-input">
          <!-- Input -->
          <input v-model="url" type="search" placeholder="Paste the Game URL" />
          <!-- Search button -->
          <a :href="`${url}/`" class="searchbar-clear"></a>
        </div>
      </form>
    </div>
  </div>
</template>

<script>
export default {
  name: "Index",
  components: {},
  // Currently I want to use this url attribute in the src of iframe but the im not sure how to pass the data to it.
  data(){
    return {
      url: ""
    }
  }
}
</script>

<style scoped>
.page-wrapper {
  padding: 1rem;
}

.searchbar-clear {
  width: 200px;
  padding: 20px;
  border: 0px solid #00B4CC;
  background: #00B4CC;
  color: white;
  border-radius: 0 10px 10px 0;
  cursor: pointer;
  font-size: 20px;
  background-image: linear-gradient(#2193b0,#00B4CC);
  box-shadow: 5px 5px 5px #888888;
}

.searchbar-clear:hover {
  opacity: 0.9;
  background-image: linear-gradient(#2193b0,#00B4CC);
}

input {
  width: calc(50% - 50px);
  border: 0px;
  padding: 20px;
  font-size: 1.3em;
  background-color: #323333;
  color: #687f7f;
  border-radius: 10px 0px 0px 10px;
  box-shadow: 5px 5px 5px #888888;
}

input:hover{
  opacity: 0.95;
}
</style>

<template>
<div class="container">
    <div class="input-group">
      <input v-model="url" class="form-control" type="search" placeholder="      Search for anything" />
      <a :href="`${url}/`" class="searchbar-clear"><i class="fa fa-search"></i></a>
    </div>
</div>
</template>

<script>
export default {
  data(){
    return {
      url: ""
    }
  }
}
</script>

<style scoped>
@import "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css";

.container {
    padding: 10px;
}
.input-group .form-control {
  background: #213246;
  border: 3px solid #2D4567;
  box-sizing: border-box;
  border-radius: 50px;
  width: 398px;
  height: 38px;
  display:grid;
  padding-left: 25px;
  margin-left: 50px;
  margin-top: 5px;
  margin-bottom: 5px;
}

.input-group a {
  position: absolute;
  top: 15px;
  left: 510px;
}

.fa-search {
  color: #808080;
}

.fa-search:hover {
  color: aqua;
}
</style>

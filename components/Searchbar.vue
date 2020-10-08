<template>
<div class="col-4 py-10 m-10">
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

.input-group .form-control {
  background: #213246;
  border: 1px solid #2D4567;
  box-sizing: border-box;
  border-radius: 50px;
  width: 550px;
}
.input-group a {
  position: absolute;
  top: 7px;
  left: 14px;
}

.fa-search {
  color: #808080;
}

.fa-search:hover {
  color: aqua;
}
</style>

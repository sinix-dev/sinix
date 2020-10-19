<template>
  <div class="container">
        <span class="rounded-circle" alt="100x100"><b>{{ title[0] }}</b></span>
        <span class="dot-badge"></span>
  </div>
</template>

<script>
export default{
  props: {
    title: String
  }
}
</script>

<style scoped>
.container {
    margin: 5px;
}
.rounded-circle {
  position: absolute;
  height: 35px;
  width: 35px;
  border-radius: 50px;
  background-color: #C4C4C4;
  display: inline-block;
}

.dot-badge{
  position: relative;;
  height: 10px;
  width: 10px;
  background-color:greenyellow;
  border-radius: 50%;
  left: 19px;
  top: 10px;
}

b {
  position: relative;
  top: 8px;
  left: 13px;
}
</style>>

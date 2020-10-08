<template>
  <div class="m-2">
      <div class="row">
          <span class="rounded-circle" alt="100x100"><b>{{ title[0] }}</b></span>
          <span class="dot-badge"></span>
      </div>
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
.rounded-circle {
  height: 35px;
  width: 35px;
  background-color: #C4C4C4;
  display: inline-block;
}

.dot-badge{
  position:absolute;
  height: 10px;
  width: 10px;
  background-color:greenyellow;
  border-radius: 50%;
}

b {
  position: relative;
  top: 6px;
  left: 12px;
}
</style>>

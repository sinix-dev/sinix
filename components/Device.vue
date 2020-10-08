<template>
  <div class="mt-2 p-0">
      <div class="row ml-1 mb-1">
          <span class="dot"><b>{{ title[0] }}</b></span>
          <p>{{ title }}</p>
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
.dot {
  height: 35px;
  width: 35px;
  background-color: #ffac41;
  border-radius: 50%;
  display: inline-block;
}

b {
    position: relative;
    top: 6px;
    left: 12px;
}

p {
  position: relative;
  top:6px;
  left: 4px;
}
</style>>

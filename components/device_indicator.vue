<template>
  <div class="indicator-wrapper">
    <div class="indicator">
      {{ username[0].toUpperCase() }}
    </div>
    <span class="status" />
  </div>
</template>

<script>
export default {
  props: {
    username: {
      type: String,
      required: true
    }
  }
}
</script>

<style scoped>
.indicator-wrapper {
  margin-top: 8px;
}

.indicator {
  width: 30px;
  height: 30px;
  display: flex;
  color: #4A4A4A;
  font-size: 15px;
  font-weight: bold;
  border-radius: 50%;
  background: #FFD5D5;
  align-items: center;
  justify-content: center;
}

.status {
  right: 6px;
  width: 8px;
  height: 8px;
  display: block;
  margin-top: -10px;
  position: absolute;
  border-radius: 50%;
  background: #2EA62C;
}
</style>

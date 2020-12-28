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
  width: 40px;
  height: 40px;
  display: flex;
  color: #4A4A4A;
  font-size: 22px;
  font-weight: bold;
  border-radius: 50%;
  background: #C4C4C4;
  align-items: center;
  justify-content: center;
}

.status {
  right: 8px;
  width: 12px;
  height: 12px;
  display: block;
  margin-top: -12px;
  position: absolute;
  border-radius: 50%;
  background: #2EA62C;
}
</style>

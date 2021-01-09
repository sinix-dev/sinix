<template>
  <div class="indicator-wrapper">
    <div class="indicator">
      {{ username[0].toUpperCase() }}
    </div>
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
  font-size: 15px;
  font-weight: bold;
  border-radius: 50%;
  align-items: center;
  color: var(--color);
  background: var(--bg);
  justify-content: center;
  border: 1px solid var(--color-primary);
}
</style>

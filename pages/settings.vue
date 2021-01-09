<template>
  <div class="page-wrapper">
    <h3>Settings</h3>

    <div class="version">
      <span>Sinix v{{ version }} </span>
    </div>
  </div>
</template>

<script>
import { version } from "~/package.json"

export default {
  name: "Settings",
  data(){
    return {
      version
    }
  }
}
</script>

<style scoped>
.page-wrapper {
  padding: 1rem;
}

.version {
  color: #777;
  font-size: 12px;
  margin-top: 10px;
}
</style>

<template>
  <div class="navbar">
    <div>
      <img class="navbar-logo navbar-item" src="/images/logo-light.png">
    </div>
    <div style="flex: 1" />
    <div class="navbar-item">
      Settings
    </div>
  </div>
</template>

<style scoped>
.navbar {
  height: 50px;
  width: 100%;
  display: flex;
  align-items: center;
  background: var(--bg-color);
}

.navbar-item {
  margin: 0 0.75rem;
}

.navbar-logo {
  height: 1rem;
}
</style>

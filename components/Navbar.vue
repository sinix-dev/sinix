<template>
  <div class="navbar row">
    <div class="col-2">
      <img class="navbar-logo navbar-item" src="/images/logo-light.png">
    </div>
    <div class="col-9">
      <Searchbar />
    </div>
    <div class="col-1 pl-10">
      Settings
    </div>
  </div>
</template>

<style scoped>
.navbar-logo {
  height: 1rem;
}

.navbar {
   background:rgba(21, 36, 56, 0.86);
   height: 65px;
   width: 100%;
}
</style>

<template>
  <div class="p-3">
    <div class="item">
      Explore
    </div>
    <div class="item">
      Installed
    </div>
    <div class="item">
      Browse
    </div>
  </div>
</template>

<style scoped>
.item {
  @apply uppercase text-sm;
  @apply cursor-pointer;
  @apply w-full px-4 py-1 rounded-full;
}

.item:hover {
  background-color: #dc143c14;
}
</style>

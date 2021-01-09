<template>
  <div class="card" />
</template>

<style>
.card {
  animation: 0.5s appear;
  width: 399px;
  height: 315px;
  background: white;
  border-radius: 5px;
  filter: drop-shadow(5px 5px 4px rgba(0, 0, 0, 0.25));
}

@keyframes appear {
  0% {
    opacity: 0;
  }
}
</style>

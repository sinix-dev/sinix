<template>
  <div class="layout">
    <Navbar />
    <div class="view">
      <div class="sidebar"></div>
      <div class="content">
        <Nuxt />
      </div>
    </div>
  </div>
</template>

<style>
:root {
  --text-black: #24292E;
}

@font-face {
  font-family: 'Roboto';
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url(../static/fonts/Roboto-Regular.ttf);
}

html, body, #__nuxt, #__layout {
  color: var(--text-black);
  font-family: 'Roboto', sans-serif;

  @apply w-full h-full
}

.layout {
  grid-template-rows: 50px auto;

  @apply grid h-full;
}

.view {
  grid-template-columns: 250px auto;

  @apply grid;
}
</style>

<template>
  <div class="layout">
    <Navbar />
    <div class="view">
      <div class="sidebar">
        <Sidebar />
      </div>
      <div class="content">
        <Nuxt />
      </div>
    </div>
  </div>
</template>

<style>
:root {
  --bg-color-opaque: #FFFFFF;
  --bg-color: rgb(255, 255, 255);
  --bg-color-trans: rgba(255, 255, 255, 0.98);
  --border-color: #FFFFFF;
}

html, body, #__nuxt, #__layout {
  font-family: 'Roboto', sans-serif;

  @apply w-full h-full text-gray-900;
}

input {
  outline: none;
  border: 1px solid #EEE;
}

button {
  outline: none;
}

@font-face {
  font-family: 'Roboto';
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url(../static/fonts/Roboto-Regular.ttf);
}

.layout {
  grid-template-rows: 50px auto;
  background: url(../static/images/background.jpg);

  @apply grid h-full bg-fixed bg-center bg-no-repeat bg-cover;
}

.view {
  grid-template-columns: 250px auto;

  @apply grid;
}

.content {
  background: var(--bg-color-trans);
  border: 1px solid var(--border-color);
}

.link {
  color: crimson;

  @apply cursor-pointer;
}

.link:hover {
  @apply underline;
}
</style>

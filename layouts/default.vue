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
  --bg-color-opaque: rgba(21, 36, 56, 1);
  --bg-color: rgba(21, 36, 56, 0.86);
  --bg-color-trans: rgba(21, 36, 56, 0.79);
  --border-color: #223346;
}

html, body, #__nuxt, #__layout {
  font-family: 'Roboto', sans-serif;

  @apply w-full h-full tracking-wide text-gray-100;
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

a, .link {
  color: crimson;

  @apply cursor-pointer;
}

a:hover, .link:hover {
  @apply underline;
}
</style>

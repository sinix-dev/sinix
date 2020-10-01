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
  --text-color: #1A202C;
  --bg-color-opaque: #FFFFFF;
  --bg-color: rgb(255, 255, 255);
  --bg-color-trans: rgba(255, 255, 255, 0.99);
  --border-color: #EDEEEF;
}

html, body, #__nuxt, #__layout {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  color: var(--text-color);
  letter-spacing: 0.025em;
  font-family: 'Roboto', sans-serif;
}

@font-face {
  font-family: 'Roboto';
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url(../static/fonts/Roboto-Regular.ttf);
}

h1, h2, h3, h4, h5, h6, p {
  margin: 0;
  margin-bottom: 0.25rem;
}

a {
  text-decoration: none;
  color: var(--text-color);
}

.layout {
  display: grid;
  height: 100%;
  background-size: cover;
  background-position: center;
  background-attachment: fixed;
  background-repeat: no-repeat;
  grid-template-rows: 50px auto;
  background: url(../static/images/background.jpg);
}

.view {
  display: grid;
  border-top: 1px solid var(--border-color);
  grid-template-columns: 250px auto;
}

.sidebar {
  background: var(--bg-color);
  border-right: 0.1px solid var(--border-color);
}

.content {
  background: var(--bg-color-trans);
}

.link {
  color: crimson;
  cursor: pointer;
}

.link:hover {
  text-decoration: underline;
}
</style>

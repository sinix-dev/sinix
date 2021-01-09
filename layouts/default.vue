<template>
  <div class="layout">
    <div class="sidebar">
      <Sidebar />
    </div>
    <div class="content">
      <Nuxt />
    </div>
    <div class="rightbar">
      <Rightbar />
    </div>
  </div>
</template>

<style>
:root {
  --text-color: #1A202C;
  --bg-color-opaque: #FFFFFF;
  --bg-color: rgb(255, 255, 255);
  --bg-color-trans: #FBF9F9;
  --border-color: #EDEEEF;
}

html, body, #__nuxt, #__layout {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  color: var(--text-color);
  font-family: 'Helvetica';
}

@font-face {
  font-family: 'Helvetica';
  font-weight: normal;
  src: url(~assets/fonts/Helvetica.ttf);
}

@font-face {
  font-family: 'Helvetica';
  font-weight: bold;
  src: url(~assets/fonts/Helvetica-Bold.ttf);
}

@font-face {
  font-family: 'Helvetica';
  font-weight: lighter;
  src: url(~assets/fonts/Helvetica-Light.ttf);
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
  height: 100vh;
  display: grid;
  grid-template-columns: 228px auto 45px;
}

.sidebar {
  color: #24292E;
  background: #FFE7E7;
}

.rightbar {
  background: var(--bg-color);
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

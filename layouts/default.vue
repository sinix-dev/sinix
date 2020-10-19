<template>
<div class = "layout">
  <div class = "row-1">
    <Navbar />
  </div>
  <div class = "row-2">
    <div class = "col-1">
      <Sidebar />
    </div>
    <div class = "col-2">
      <div class = "col-2-row-1">
        COL-2-ROW-1
      </div>
      <div class = "col-2-row-2">
        COL-2-ROW-2
      </div>
    </div>
    <div class = "col-3">
      <ActiveDevices />
    </div>
  </div>
</div>
</template>

<style>
:root {
  --text-color: #1A202C;
  --bg-color-opaque: #FFFFFF;
  --bg-color: rgba(21, 36, 56, 0.86);
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
  overflow: hidden;
  display: grid;
  height: 768px;
  width: 100%;
  margin: 0;
  grid-template-rows: 50px auto;
  grid-template-areas:
    'row-1'
    'row-2';
}

.row-1 {
  height: 50px;
  width: 100%;
  background: rgba(21, 36, 56, 0.86);
  border: 1px solid #223346;
  box-sizing: border-box;
}
.row-2 {
  display:grid;
  grid-template-columns: 246px auto 58px;
  height: 768px;
  width: 100%;
  grid-template-areas:'col-1 col-2 col-3';
}

.col-1 {
  height: 100%;
  width: 246px;
  background: rgba(21, 36, 56, 0.95);
  border: 1px solid #223146;
  box-sizing: border-box;
}

.col-2 {
  height: 100%;
  width: auto;
}

.col-2-row-1{
  height:48px;
  background: rgba(21, 36, 56, 0.86);
  border: 1px solid #223346;

}

.col-2-row-2 {
  height: 100%;
  background: rgba(21, 36, 56, 0.59);
  border: 1px solid #223346;
}

.col-3 {
  width:58px;
  height: 100%;
  background: rgba(21, 36, 56, 0.86);
}
</style>

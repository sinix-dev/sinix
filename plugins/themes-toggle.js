document.addEventListener("keydown", (e) => {
  const theme = $nuxt.$colorMode.preference
  if(e.ctrlKey && e.keyCode === 84){
    $nuxt.$colorMode.preference = theme === "light" ? "dark" : "light"
  }
})

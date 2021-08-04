<main class="flex">
  <Sidebar />
</main>

<script>
import Sidebar from "./components/Sidebar.svelte"

let name = "Sinix"
localStorage.setItem("name", "Sinix")
</script>

<style>
main {
  height: 100%;
  width: 100%;
}
</style>

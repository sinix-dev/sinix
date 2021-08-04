<main class="flex">
  <Sidebar />
  <div>
    <slot />
  </div>
</main>

<script>
import Sidebar from "../components/Sidebar.svelte"
</script>

<style>
main {
  height: 100%;
  width: 100%;
}
</style>

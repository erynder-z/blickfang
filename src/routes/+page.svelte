<script lang="ts">
  import ImageView from "../lib/components/ImageView.svelte";
  import Controls from "../lib/components/Controls.svelte";
  import ImageNameDisplay from "$lib/components/ImageNameDisplay.svelte";
  import { isSidebarVisible } from "$lib/store";
  import ExifDisplaySidebar from "$lib/components/ExifDisplaySidebar.svelte";
</script>

<main class:sidebar-hidden={!$isSidebarVisible}>
  <div class="image-section">
    <ImageView />
    <Controls />
    <ImageNameDisplay />
  </div>
  <div class="sidebar-section"><ExifDisplaySidebar /></div>
</main>

<style>
  main {
    height: 100vh;
    position: relative;
    display: grid;
    grid-template-areas: "image-section sidebar-section";
    grid-template-columns: 80% 20%;
    grid-template-rows: minmax(0, 1fr);
    transition: grid-template-columns 0.1s ease-in-out;
  }

  main.sidebar-hidden {
    grid-template-columns: 100% 0;
  }

  .image-section {
    position: relative;
  }

  .sidebar-section {
    overflow: hidden;
  }

  :global(*, *:before, *:after) {
    box-sizing: border-box;
  }

  :global(:root) {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    color: #747474;
    background-color: #241f2e;
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  :global(body) {
    margin: 0;
  }
</style>

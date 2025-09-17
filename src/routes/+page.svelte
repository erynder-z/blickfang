<script lang="ts">
  import ImageView from "../lib/components/ImageView.svelte";
  import Controls from "../lib/components/Controls.svelte";
  import Footer from "../lib/components/Footer.svelte";
</script>

<main>
  <ImageView />
  <Controls />
  <Footer />
</main>

<style>
  main {
    height: 100vh;
    position: relative;
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

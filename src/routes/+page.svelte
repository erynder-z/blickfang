<script lang="ts">
  import Header from "../lib/components/Header.svelte";
  import ImageView from "../lib/components/ImageView.svelte";
  import Controls from "../lib/components/Controls.svelte";
  import Footer from "../lib/components/Footer.svelte";
</script>

<main>
  <Header />
  <ImageView />
  <Controls />
  <Footer />
</main>

<style>
  :global(:root) {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    color: #0f0f0f;
    background-color: #f6f6f6;
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  :global(body) {
    margin: 0;
  }

  @media (prefers-color-scheme: dark) {
    :global(:root) {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>

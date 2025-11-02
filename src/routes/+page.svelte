<script lang="ts">
  import ImageView from "../lib/components/ImageView.svelte";
  import Controls from "$lib/components/Controls.svelte";
  import OptionsButton from "$lib/components/OptionsButton.svelte";
  import ImageNameDisplay from "$lib/components/ImageNameDisplay.svelte";
</script>

<main>
  <ImageView />
  <OptionsButton />
  <Controls />
  <ImageNameDisplay />
</main>

<style>
  @font-face {
    font-family: SUSE Mono;
    src: url(../assets/SUSEMono-VariableFont_wght.ttf) format("truetype");
  }

  main {
    height: 100%;
    position: relative;
    height: 100%;
    width: 100%;
  }

  :global(*, *:before, *:after) {
    box-sizing: border-box;
  }

  :global(:root) {
    font-family: "SUSE Mono", monospace;
    font-optical-sizing: auto;
    font-size: 1rem;
    line-height: 1rem;
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

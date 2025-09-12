<script lang="ts">
  import { imageUrl, zoomLevel } from "$lib/store";
  import { invoke } from "@tauri-apps/api/core";

  const openFile = async () => {
    try {
      const result = await invoke<string>("open_and_read_file");
      if (result) imageUrl.set(result);
    } catch (error) {
      console.error("Failed to open and read file:", error);
    }
  };

  const previous = () => {
    // TODO
  };

  const next = () => {
    // TODO
  };

  const zoomIn = () => {
    zoomLevel.update((level) => level + 0.25);
  };

  const zoomOut = () => {
    zoomLevel.update((level) => {
      const newLevel = level - 0.25;
      return newLevel < 0.1 ? 0.1 : newLevel;
    });
  };
</script>

<div class="controls-container">
  <button on:click={openFile}>Open Image</button>
  <button on:click={previous}>Previous</button>
  <button on:click={next}>Next</button>
  <button on:click={zoomIn}>Zoom In</button>
  <button on:click={zoomOut}>Zoom Out</button>
</div>

<style>
  .controls-container {
    height: 5vh;
    display: flex;
    justify-content: center;
    gap: 1rem;
    margin: 1rem;
  }
</style>

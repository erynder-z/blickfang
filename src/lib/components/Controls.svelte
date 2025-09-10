<script lang="ts">
  import { imageUrl } from "$lib/store";
  import { invoke } from "@tauri-apps/api/core";

  const openFile = async () => {
    try {
      const result = await invoke<string>("open_and_read_file");
      if (result) {
        imageUrl.set(result);
      }
    } catch (error) {
      console.error("Failed to open and read file:", error);
    }
  };
</script>

<div class="controls-container">
  <button on:click={openFile}>Open Image</button>
  <button>Previous</button>
  <button>Next</button>
</div>

<style>
  .controls-container {
    display: flex;
    justify-content: center;
    gap: 1rem;
    margin: 1rem;
  }
</style>

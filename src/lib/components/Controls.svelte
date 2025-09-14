<script lang="ts">
  import { get } from "svelte/store";
  import { imagePath, imageUrl, zoomLevel } from "$lib/store";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  onMount(async () => {
    await listen<string>("new_image_path", (event) => {
      imagePath.set(event.payload);
    });
  });

  const openFile = async () => {
    try {
      const [newImageData, newImagePath] = await invoke<[string, string]>(
        "open_and_read_file"
      );
      if (newImageData) {
        imageUrl.set(newImageData);
        imagePath.set(newImagePath);
      }
    } catch (error) {
      console.error("Failed to open and read file:", error);
    }
  };

  const changeImage = async (direction: "next" | "previous") => {
    const currentPath = get(imagePath);
    if (!currentPath) return;

    try {
      const [newImageData, newImagePath] = await invoke<[string, string]>(
        "change_image",
        { currentPath, direction }
      );
      imageUrl.set(newImageData);
      imagePath.set(newImagePath);
    } catch (error) {
      console.error("Failed to change image:", error);
    }
  };

  const previous = () => {
    changeImage("previous");
  };

  const next = () => {
    changeImage("next");
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

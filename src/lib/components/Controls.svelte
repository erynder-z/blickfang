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
      const [newImageData, newImagePath] = await invoke<[string, string]>("open_and_read_file");
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
      const [newImageData, newImagePath] = await invoke<[string, string]>("change_image", {
        currentPath,
        direction,
      });
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
  <button on:click={openFile}>
    <img src="/file_open_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg" alt="Open" />
  </button>
  <button on:click={previous}>
    <img src="/arrow_back_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg" alt="Previous" />
  </button>
  <button on:click={next}>
    <img src="/arrow_forward_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg" alt="Next" />
  </button>
  <button on:click={zoomIn}>
    <img src="/zoom_in_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg" alt="Zoom In" />
  </button>
  <button on:click={zoomOut}>
    <img src="/zoom_out_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg" alt="Zoom Out" />
  </button>
</div>

<style>
  .controls-container {
    position: absolute;
    top: 0;
    width: 100%;
    display: flex;
    justify-content: center;
    gap: 1rem;
    padding: 1rem;
    background-color: transparent;
    z-index: 10;
  }

  button {
    background-color: rgba(67, 67, 67, 0.5);
    border: none;
    cursor: pointer;
    padding: 0.5rem;
    border-radius: 50%;
    transition: all 0.2s ease-in-out;
  }

  button:hover {
    filter: brightness(1.5);
    cursor: pointer;
  }

  img {
    width: 24px;
    height: 24px;
  }
</style>

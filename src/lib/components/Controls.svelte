<script lang="ts">
  import { get } from "svelte/store";
  import { currentDirectoryFiles, imagePath, imageUrl, zoomLevel } from "$lib/store";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let currentImageDirectory = "";

  onMount(async () => {
    await listen<string>("new_image_path", async (event) => {
      imagePath.set(event.payload);
      const path = event.payload;
      const directory = path.substring(0, path.lastIndexOf("/"));
      if (directory !== currentImageDirectory) {
        currentImageDirectory = directory;
        try {
          const files = await invoke<string[]>("get_directory_files", {
            path: path,
          });
          currentDirectoryFiles.set(files.sort());
        } catch (error) {
          console.error("Failed to get directory files:", error);
        }
      }
    });
  });

  const openFile = async () => {
    try {
      const result = await invoke<string>("open_and_read_file");
      if (result) imageUrl.set(result);
    } catch (error) {
      console.error("Failed to open and read file:", error);
    }
  };

  const changeImage = async (direction: "next" | "previous") => {
    const currentPath = get(imagePath);
    const files = get(currentDirectoryFiles);

    if (!currentPath || files.length <= 1) return;

    try {
      const currentIndex = files.indexOf(currentPath);
      if (currentIndex === -1) return;

      const nextIndex =
        (currentIndex + (direction === "next" ? 1 : -1) + files.length) % files.length;

      const nextImagePath = files[nextIndex];
      const newImageData = await invoke<string>("read_image_file", {
        path: nextImagePath,
      });

      imageUrl.set(newImageData);
      imagePath.set(nextImagePath);
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

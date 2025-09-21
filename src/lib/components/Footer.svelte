<script lang="ts">
  import { imagePath, imageExif } from "$lib/store";
</script>

<footer>
  {#if $imagePath}
    <span>{$imagePath}</span>
  {/if}
  {#if $imageExif}
    <span>{$imageExif}</span>
  {/if}
</footer>

<style>
  footer {
    position: absolute;
    bottom: 0;
    width: 100%;
    padding: 1rem;
    text-align: center;
    background-color: transparent;
    color: white;
    z-index: 10;
    display: flex;
    justify-content: center;
    gap: 1rem;
  }
</style>
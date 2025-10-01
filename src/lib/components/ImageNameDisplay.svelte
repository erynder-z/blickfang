<script lang="ts">
  import { imagePath } from "$lib/store";

  $: imageName = $imagePath ? $imagePath.split(/[/\\]/).pop() : undefined;
</script>

<div>
  {#if imageName}
    <p>{imageName}</p>
  {/if}
</div>

<style>
  div {
    position: absolute;
    bottom: 0;
    width: 100%;
    text-align: center;
    background-color: transparent;
    color: white;
    z-index: 10;
    display: flex;
    justify-content: center;
  }

  p {
    background-color: var(--color-shadow);
    color: white;
    padding: 0.5rem 0.75rem;
    border-radius: 25%/100%;
  }
</style>

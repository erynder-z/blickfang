<script lang="ts">
  import { imagePath } from "$lib/store";
  import { t } from "$lib/i18n";

  $: imageName = $imagePath ? $imagePath.split("/").pop() : undefined;
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
    background-color: rgba(0, 0, 0, 0.25);
    color: white;
    padding: 0.5rem 0.75rem;
    border-radius: 25%/100%;
  }
</style>

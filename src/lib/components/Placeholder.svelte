<script lang="ts">
  export let resizing = false;
</script>

<div class="placeholder" class:resizing></div>

<style>
  .placeholder {
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg, #333, #555);
    opacity: 0;
    pointer-events: none;
    transition: opacity 120ms ease-out;
    z-index: 1;
  }
  .placeholder.resizing {
    opacity: 1;
  }
</style>

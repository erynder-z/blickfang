<script lang="ts">
  import { t } from "$lib/i18n";
</script>

<div class="options-container">
  <h1>{$t["options.title"]}</h1>
  <button>{$t["options.hotkeys"]}</button>
  <button>{$t["options.language"]}</button>
</div>

<style>
  .options-container {
    padding: 1rem;
    color: #e3e3e3;
    height: 100%;
    overflow-y: auto;
  }

  button {
    padding: 0.5rem;
    border: none;
    border-radius: 0.5rem;
    background-color: rgba(255, 255, 255, 0.2);
    color: #e3e3e3;
    cursor: pointer;
    font-weight: bold;
  }
</style>

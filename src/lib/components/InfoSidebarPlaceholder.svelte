<script lang="ts">
  import { t } from "$lib/utils/i18n";
</script>

<div class="placeholder-container">
  <svg
    xmlns="http://www.w3.org/2000/svg"
    height="24px"
    viewBox="0 -960 960 960"
    width="24px"
    fill="currentColor"
    ><path
      d="m840-234-80-80v-446H314l-80-80h526q33 0 56.5 23.5T840-760v526ZM792-56l-64-64H200q-33 0-56.5-23.5T120-200v-528l-64-64 56-56 736 736-56 56ZM240-280l120-160 90 120 33-44-283-283v447h447l-80-80H240Zm297-257ZM424-424Z"
    /></svg
  >
  <p>{$t["placeholder.no-image"]}</p>
</div>

<style>
  .placeholder-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100%;
    gap: 1rem;
    color: var(--color-text-secondary);
    padding: 1rem;
    text-align: center;
  }

  svg {
    width: 4rem;
    height: 4rem;
    fill: var(--color-text-secondary);
  }
</style>

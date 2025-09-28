<script lang="ts">
  import { lang } from '$lib/actions/lang';
  import { locale } from '$lib/i18n';
  import LanguageMenu from '$lib/components/LanguageMenu.svelte';
  import ExifDisplaySidebar from "$lib/components/ExifDisplaySidebar.svelte";
  import OptionsDisplaySidebar from "$lib/components/OptionsDisplaySidebar.svelte";
</script>

<svelte:body use:lang={$locale} />

<LanguageMenu />

<div class="app-container">
  <ExifDisplaySidebar />
  <OptionsDisplaySidebar />

  <slot />
</div>

<style>
  .app-container {
    position: relative;
    height: 100vh;
    overflow: hidden;
  }
</style>
<script lang="ts">
  import { lang } from '$lib/actions/lang';
  import { locale } from '$lib/i18n';
  import LanguageMenu from '$lib/components/LanguageMenu.svelte';
</script>

<svelte:body use:lang={$locale} />

<LanguageMenu />

<slot />

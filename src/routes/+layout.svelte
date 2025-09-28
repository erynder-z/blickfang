<script lang="ts">
  import { lang } from '$lib/actions/lang';
  import { locale } from '$lib/i18n';
</script>

<svelte:body use:lang={$locale} />

<slot />

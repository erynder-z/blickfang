<script lang="ts">
  import { tooltip } from "$lib/stores/ui";
  import Tooltip from "./Tooltip.svelte";
</script>

<Tooltip text={$tooltip.text} visible={$tooltip.visible} targetRect={$tooltip.targetRect} />

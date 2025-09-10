<script lang="ts">
	import { imageUrl } from '$lib/store';
</script>

<div class="image-view-container">
	{#if $imageUrl}
		<img src={$imageUrl} alt="Selected" />
	{:else}
		<p>Select an image to view</p>
	{/if}
</div>

<style>
	.image-view-container {
		display: flex;
		justify-content: center;
		align-items: center;
		flex-grow: 1;
		margin: 1rem;
		min-height: 0;
	}
	img {
		max-width: 100%;
		max-height: 100%;
		object-fit: contain;
	}
</style>

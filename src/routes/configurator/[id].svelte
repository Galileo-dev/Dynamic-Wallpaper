<script>
	import BackButton from '$lib/BackButton.svelte';
</script>

<div class="header">
	<BackButton />
	<h1 class="header__text">Set Wallpaper</h1>
</div>

<style>
	.header {
		display: flex;
		flex-direction: row;
		align-items: center;
	}

	.header__text {
		padding: 0;
		margin: 0;
		margin-left: 2rem;
	}
</style>

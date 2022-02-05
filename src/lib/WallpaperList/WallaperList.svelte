<script>
	import Button from '$lib/button.svelte';
	import AddNewCard from './AddNewCard.svelte';

	import WallpaperCard from './WallpaperCard.svelte';
	let template_item = { id: '12312abc', imagePath: '../pwpoe/woapk.png' };
	export let items = [];
	for (let i = 0; i < 20; i++) {
		items.push(template_item);
	}
</script>

<h3>Past Wallpapers</h3>
<div class="scroll" />
<div class="item-list">
	{#each items as item}
		<WallpaperCard
			image={'https://source.unsplash.com/random/?sexy'}
			width="500px"
			height="300px"
			name="Usplash"
			{item}
		/>
	{/each}
	<!-- <AddNewCard width="500px" height="300px" name="A New Wallpaper" /> -->
</div>

<style>
	.scroll {
		/* overflow-x: hidden;
		overflow-y: auto;
		/* max-height: 100vh; */
		margin: 5px;
		padding: 5px;
		width: 500px;
		height: 110px;
		overflow: auto;
		text-align: justify;
	}

	div::-webkit-scrollbar {
		width: 0.25rem;
	}

	div::-webkit-scrollbar-track {
		background-color: var(--main-text-color);
		border-radius: 50px;
	}

	div::-webkit-scrollbar-thumb {
		background-color: var(--accent-color);
		border-radius: 50px;
	}

	.item-list {
		display: grid;
		justify-content: space-evenly;
		justify-self: center;
		align-self: center;
		grid-template-columns: repeat(auto-fit, 400px);
		grid-row-gap: 1rem;
		grid-column-gap: 2rem;
		width: 95%;
		height: 100%;
	}
</style>

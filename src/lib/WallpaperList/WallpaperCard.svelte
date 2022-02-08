<script>
	export let image;
	export let name;
	export let width;
	export let height;
	export let item;
</script>

<div class="container">
	<a> <img src={image} alt="wallpaper" {height} {width} /> </a>
	<!-- href={'/configurator/' + item.id} -->
	<div class="description">
		<p>added: 19-06-2004</p>
		<h2>{name}</h2>
	</div>
</div>

<style>
	.container {
		display: flex;
		flex-direction: column;
		padding: 1rem;
	}

	img {
		object-fit: cover;
		border-radius: 1rem;
		margin-bottom: 1rem;
	}

	h2 {
		font-size: 0.9rem;
	}

	p {
		font-size: 0.8rem;
	}

	.description {
		height: 5vh;
	}
</style>

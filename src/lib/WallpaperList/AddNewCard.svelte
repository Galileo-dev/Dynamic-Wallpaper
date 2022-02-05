<script>
	export let name;
	export let width;
	export let height;
</script>

<div class="container">
	<a href={'/configurator/'} />

	<div class="description">
		<p>added: 19-06-2004</p>
		<h2>{name}</h2>
	</div>
</div>

<style>
	.container {
		display: flex;
		flex-direction: column;
		justify-content: space-around;
		padding: 1rem;
	}

	img {
		object-fit: cover;
		border-radius: 1rem;
		margin-bottom: 1rem;
		width: 100%;
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

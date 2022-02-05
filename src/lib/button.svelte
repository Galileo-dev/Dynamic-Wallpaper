<script lang="ts">
	export let text = 'Create';
</script>

<a on:click class="disable-select">{text}</a>

<style>
	a {
		padding: 10px;
		border: 3px solid var(--main-text-color);
		box-sizing: border-box;
		display: inline-block;
		font-size: 16px;
		font-weight: 600;
		letter-spacing: 0.5px;
		padding: 8px 23px;
		border-radius: 2px;
		background-color: transparent;
		text-align: center;
	}

	a:hover {
		border-color: var(--main-hover-color);
		color: var(--main-hover-color);
	}

	a:active {
		border-color: var(--main-hover-color);
		background-color: var(--main-hover-color);
		color: var(--pure-white);
	}

	.disable-select {
		user-select: none; /* supported by Chrome and Opera */
		-webkit-user-select: none; /* Safari */
		-khtml-user-select: none; /* Konqueror HTML */
		-moz-user-select: none; /* Firefox */
		-ms-user-select: none; /* Internet Explorer/Edge */
	}
</style>

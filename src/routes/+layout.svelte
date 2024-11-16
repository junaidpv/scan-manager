<script>
	let { children } = $props();
</script>

<svelte:head>
	<link rel="stylesheet" href="/bootstrap/css/bootstrap.min.css" />
    <link rel="stylesheet" href="/bootstrap-icons/font/bootstrap-icons.min.css">
</svelte:head>

{@render children()}

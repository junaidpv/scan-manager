<script>
	let { children } = $props();
</script>

<svelte:head>
	<link rel="stylesheet" href="/bootstrap/css/bootstrap.min.css" />
    <link rel="stylesheet" href="/bootstrap-icons/font/bootstrap-icons.min.css">
	<script src="/bootstrap/js/bootstrap.bundle.min.js" ></script>
</svelte:head>

{@render children()}

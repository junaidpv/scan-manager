<script>
    import { page } from '$app/stores'
    const name = $page.url.searchParams.get('name') || 'NoName';
</script>
<h1>I am misc with name {name}</h1>
<script lang="ts">
    import Thumbnail from "./Thumbnail.svelte";

    let {thumbnails} = $props();
</script>
<div>
    {#each thumbnails as thumbnail}
        <Thumbnail {...thumbnail} />
    {/each}
</div>
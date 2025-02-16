<script lang="ts">
    import Thumbnail from "./Thumbnail.svelte";

    let {thumbnails} = $props();
</script>
<div>
    {#if thumbnails.length == 0}
        <p>No thumbnails</p>
    {:else}
        {#each thumbnails as thumbnail}
            <Thumbnail {...thumbnail} />
        {/each}
    {/if}
</div>
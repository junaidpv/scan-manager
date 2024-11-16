<script lang="ts">
    import Thumbnail from "./Thumbnail.svelte";
    interface ThumbnailItem {
		title: string;
        description: string;
	}

    interface Props {
        items: Array<ThumbnailItem>;
    }
    let { items }: Props = $props();
</script>
<div>
    {#each items as item}
        <Thumbnail {...item}/>
    {/each}
</div>
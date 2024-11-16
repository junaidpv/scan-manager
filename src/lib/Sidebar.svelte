<script lang="ts">
    import Thumbnail from "./Thumbnail.svelte";
    interface ThumbnailItem {
		title: string;
        description: string;
        button_color: string;
        icon: string | null;
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
<script lang="ts">
    import { path } from "@tauri-apps/api";
    import Thumbnail from "./Thumbnail.svelte";
    interface Props {
        image_infos: ImageInfo[];
    }

    let {image_infos}: Props = $props();
</script>
<div>
    {#if image_infos.length == 0}
        <p>No thumbnails</p>
    {:else}
        {#each image_infos as image_info}
            <Thumbnail {...image_info} />
        {/each}
    {/if}
</div>
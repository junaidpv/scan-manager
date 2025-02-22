<script lang="ts">
    import { emit } from "@tauri-apps/api/event";
    import { convertFileSrc } from '@tauri-apps/api/core';
    import { path } from "@tauri-apps/api";

    let image_info: ImageInfo = $props();
    
    function select_image() {
        emit('image-selected', {
            image_info: image_info,
        })
    }
</script>
<div>
    <button onclick={select_image}>
    <figure>
        <img src={convertFileSrc(image_info.path)} class="img-thumbnail" alt='' loading="lazy" />
        <figcaption>Test</figcaption>
    </figure>
    </button>
</div>
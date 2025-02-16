<script lang="ts">
    import { emit } from "@tauri-apps/api/event";
    import { convertFileSrc } from '@tauri-apps/api/core';

    let {src, alt, name } = $props();
    
    function select_image() {
        emit('image-selected', {
            src: src,
        })
    }
</script>
<div>
    <button onclick={select_image}>
    <figure>
        <img src={convertFileSrc(src)} class="img-thumbnail" alt={alt} />
        <figcaption>{name}</figcaption>
    </figure>
    </button>
</div>
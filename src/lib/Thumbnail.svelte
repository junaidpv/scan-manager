<script lang="ts">
    import { emit } from "@tauri-apps/api/event";

    let {src, alt } = $props();
    
    function select_image() {
        emit('image-selected', {
            src: src,
        })
    }
</script>
<div>
    <button onclick={select_image}><img src={src} class="img-thumbnail" alt={alt} /></button>
</div>
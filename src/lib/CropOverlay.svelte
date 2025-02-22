<script lang="ts">
    interface CropOverlayProps {
        image_width: number;
        image_height: number;
    }
    let crop_overlay_props: CropOverlayProps = $props();

    let start_x = 500;
    let start_y = 500;
    let width = 500;
    let height = 500;
</script>
<svg class="crop-overlay" style="background-color: transparent;">
    <mask id="mask">
        <rect fill="white" width="100%" height="100%"/>
        <rect fill="black" x={start_x} y={start_y} width={width} height={height} /> <!-- Transparent cutout -->
    </mask>
    <rect mask="url(#mask)" fill="rgba(0, 0, 0, 0.2)" width="100%" height="100%"/>
</svg>
<style>
    .crop-overlay {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
    }
</style>
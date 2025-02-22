<script lang="ts">
    let start_x = 5;
    let start_y = 5;
    let width = 105;
    let height = 105;
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
<script lang="ts">
    interface GridOverlayProps {
        // Size of the grid in pixels
        grid_size: number;
    }
    let { grid_size }: GridOverlayProps = $props();
</script>
<div class="grid-overlay" style="background-size: {grid_size}px {grid_size}px;"></div>
<style>
    .grid-overlay {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background:
        linear-gradient(to right, rgba(0, 0, 0, 0.1) 1px, transparent 1px),
        linear-gradient(to bottom, rgba(0, 0, 0, 0.1) 1px, transparent 1px);
      pointer-events: none; /* Ensure the overlay doesn't block interactions */
    }
</style>
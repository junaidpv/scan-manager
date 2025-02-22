<script lang="ts">

    interface Props {
        step: string;
        step_settings: StepSettings;
    }
    let {step, step_settings = $bindable()}: Props = $props();

    let step_map: { [key: string]: string } = {
        'fix_orientation': 'Roate',
        'deskew': 'Deskew',
        'create_frame': 'Create Frame',
        'crop': 'Crop'
    }
    // Function to rotate the angle by 90 degrees
    function rotateAngle(angle: number) {
        step_settings.rotate_angle = (step_settings.rotate_angle + angle) % 360; // Increment and wrap around at 360
    }
</script>
<div>
<fieldset class="text-bg-light border border-dark rounded-1 my-2 p-2">
    <legend>{step_map[step]}</legend>
    <div class="text-center row gy-1">
        {#if step == 'fix_orientation'}
            <div class="col-12">
                <button type="button" class="btn btn-primary" aria-label="Roate Counter Clockwise" onclick={() => {rotateAngle(-90)}}>
                    <i class="bi bi-arrow-counterclockwise"></i>
                </button>
                <strong>{step_settings.rotate_angle}°</strong>
                <button type="button" class="btn btn-primary" aria-label="Roate Clockwise" onclick={() => {rotateAngle(90)}}>
                    <i class="bi bi-arrow-clockwise"></i>
                </button>
            </div>
            <div class="col-12">
                <button type="button" class="btn btn-secondary" aria-label="Reset" onclick={() => {step_settings.rotate_angle = 0;}}>Reset</button>
            </div>
        {/if}
        {#if step == 'deskew'}
            <div class="col-12">
                <input type="radio" class="btn-check" name="options" id="auto" autocomplete="off" checked={step_settings.deskew_mode == 'auto'} onclick="{() => {step_settings.deskew_mode = 'auto'}}">
                <label class="btn" for="auto">Auto</label>

                <input type="radio" class="btn-check" name="options" id="manual" autocomplete="off" checked={step_settings.deskew_mode == 'manual'} onclick="{() => {step_settings.deskew_mode = 'manual'}}">
                <label class="btn" for="manual">Manual</label>
                {#if step_settings.deskew_mode == 'manual'}
                <div class="pt-2">
                    <input type="number" class="" min="-45" max="45" step="0.1" value={step_settings.deskew_value}>
                </div>
                {/if}
            </div>
            <div class="col-12">
                <button type="button" class="btn btn-secondary" aria-label="Reset">Reset</button>
            </div>
        {/if}
        {#if step == 'create_frame'}
        <div class="col-12">
            <strong>Frame Ratio (W:H)</strong>
            <div class="row">
                <div class="col-6">
                    <input id="frame-ratio-width" class="w-100" type="number" min="1" max="100" step="1" placeholder="1" aria-label="Frame Ratio Width" value={step_settings.frame_ratio_width}>
                </div>
                <div class="col-6">
                    <input id="frame-ratio-height" class="w-100" type="number" min="1" max="100" step="1" placeholder="1" aria-label="Frame Ratio Height" value={step_settings.frame_ratio_height}>
                </div>
            </div>
        </div>
        {/if}
        {#if step == 'crop'}
            <p>Crop</p>
        {/if}
    </div>
</fieldset>
</div>
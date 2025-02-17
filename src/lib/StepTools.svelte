<script lang="ts">

    interface Props {
        step: string;
        deskew_mode: string;
        deskew_value: number;
        frame_ratio_width: number;
        frame_ratio_height: number;
    }
    let {step, deskew_mode, deskew_value, frame_ratio_width, frame_ratio_height}: Props = $props();

    let step_map: { [key: string]: string } = {
        'fix_orientation': 'Roate',
        'deskew': 'Deskew',
        'create_frame': 'Create Frame',
        'crop': 'Crop'
    }
    
</script>
<div>
<fieldset class="text-bg-light border border-dark rounded-1 my-2 p-2">
    <legend>{step_map[step]}</legend>
    <div class="text-center row gy-1">
        {#if step == 'fix_orientation'}
            <div class="col-12">
                <button type="button" class="btn btn-primary" aria-label="Roate Counter Clockwise"><i class="bi bi-arrow-counterclockwise"></i></button>
                <button type="button" class="btn btn-primary" aria-label="Roate Counter Clockwise"><i class="bi bi-arrow-clockwise"></i></button>
            </div>
            <div class="col-12">
                <button type="button" class="btn btn-secondary" aria-label="Reset">Reset</button>
            </div>
        {/if}
        {#if step == 'deskew'}
            <div class="col-12">
                <input type="radio" class="btn-check" name="options" id="auto" autocomplete="off" checked={deskew_mode == 'auto'} onclick="{() => {deskew_mode = 'auto'}}">
                <label class="btn" for="auto">Auto</label>

                <input type="radio" class="btn-check" name="options" id="manual" autocomplete="off" checked={deskew_mode == 'manual'} onclick="{() => {deskew_mode = 'manual'}}">
                <label class="btn" for="manual">Manual</label>
                {#if deskew_mode == 'manual'}
                    <input type="number" class="form-range" min="-45" max="45" step="0.1" value={deskew_value}>
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
                    <input id="frame-ratio-width" class="w-100" type="number" min="1" max="100" step="1" placeholder="1" aria-label="Frame Ratio Width" value={frame_ratio_width}>
                </div>
                <div class="col-6">
                    <input id="frame-ratio-height" class="w-100" type="number" min="1" max="100" step="1" placeholder="1" aria-label="Frame Ratio Height" value={frame_ratio_height}>
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
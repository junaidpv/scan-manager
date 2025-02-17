<script lang="ts">

    let active_step = $state('fix_orientation');
    let steps = [
        {
            name: 'fix_orientation',
            title: 'Fix orientation',
            description: 'Fix the orientations of the images',
            icon: 'bi-arrow-repeat',
        },
        {
            name: 'deskew',
            title: 'Deskew',
            description: 'Deskew the image',
            icon: 'bi-bounding-box-circles',
        },
        {
            name: 'create_frame',
            title: 'Create Frame',
            description: 'Create a frame around the image',
            icon: 'bi-crop',
        },
        {
            name: 'crop',
            title: 'Crop',
            description: 'Crop the image',
            icon: 'bi-crop',
        }
    ];
    function select_step(step_name: string) {
        active_step = step_name;
    }
</script>
<div class="d-grid mx-auto">
    {#each steps as step}
        <button class="btn btn-secondary text-start rounded-0 {step.name === active_step ? 'active' : ''}" type="button" onclick={(event) => {select_step(step.name)}}>
            <i class="bi {step.icon}"></i>
            {step.title}
        </button>
    {/each}
</div>
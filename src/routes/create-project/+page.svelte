<script>

    import { open } from '@tauri-apps/plugin-dialog';
    // when using `"withGlobalTauri": true`, you may use
    // const { open } = window.__TAURI__.dialog;

    let project_location = $state();

    let project_location_button_label = $derived.by(() => {
        if (project_location) {
            return 'Change Folder';
        }
        else {
            return 'Select Folder';
        }
    });
    let submit_button_label = $derived.by(() => {
        if (project_location) {
            return 'Update';
        }
        else {
            return 'Create';
        }
    });

    // Open a dialog
    async function openProjectLocation() {
        const file = await open({
        multiple: false,
        directory: true,
        });
        if (file) {
            project_location = file;
        }
    }
    
</script>

<div class="container-fluid">
<form>
<div class="mb-3">
    <label for="projectName" class="form-label">Project Name</label>
    <input type="project_name" class="form-control" id="projectName" aria-describedby="projectNameHelp">
    <div id="projectNameHelp" class="form-text">A name for the project.</div>
</div>
<div class="mb-3">
    <label for="scanLocation" class="form-label">Scan Location</label>
    <div>{project_location}</div>
    <input type="button" class="form-control" id="scanLocation" value="{project_location_button_label}" onclick={openProjectLocation}>
</div>
<div class="mb-3 f">
    <label for="projectDescription" class="form-label">Description</label>
    <textarea id="projectDescription" name="project_description" class="form-control" aria-describedby="projectDescriptionHelp"></textarea>
    <div id="projectDescriptionHelp" class="form-text">A more detailed description about this project.</div>
</div>
<button type="submit" class="btn btn-primary">{submit_button_label}</button>
</form>
</div>
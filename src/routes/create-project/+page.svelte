<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { getCurrentWebview } from "@tauri-apps/api/webview";
    import { open } from '@tauri-apps/plugin-dialog';
    import { emit, listen } from '@tauri-apps/api/event'
    // when using `"withGlobalTauri": true`, you may use
    // const { open } = window.__TAURI__.dialog;

    let project_name = $state('test');
    let project_location = $state('/test');
    let project_description = $state('test desc');
    let error = $state(false);
    let error_message = $state('');

    let project_location_button_label = $derived.by(() => {
        if (project_location) {
            return 'Change Folder';
        }
        else {
            return 'Select Folder';
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
    
    // @ts-ignore
    async function submit(event) {
        // event.preventDefault();
        console.log(project_name, project_location, project_description);
        let now_timestamp = new Date().getTime();
            let project: ProjectItem & Record<string, unknown> = {
                name: project_name,
                scan_location: project_location,
                description: project_description,
                created_at: now_timestamp,
                updated_at: now_timestamp,
            }
        let response_string = await invoke("create_project", project) as string;
        let response = JSON.parse(response_string);
        console.log(response);
        if (response.result) {
            emit('project-selected', {
                projectData: project,
            })
            .then(() => {
                getCurrentWebview().window.close();
            });
        }
        else {
            error = true;
            error_message = response.message;
        }
    }

    function closeWindow() {
        getCurrentWebview().window.destroy();
    }
</script>

<div class="container-fluid">
<form>
<div class="mb-3">
    <label for="projectName" class="form-label">Project Name</label>
    <input type="project_name" class="form-control" id="projectName" aria-describedby="projectNameHelp" bind:value={project_name}
    class:is-invalid={error}
    >
    <div id="invalidCheck3Feedback" class="invalid-feedback">
        { error_message }
    </div>
    <div id="projectNameHelp" class="form-text">A name for the project.</div>
</div>
<div class="mb-3">
    <label for="scanLocation" class="form-label">Scan Location</label>
    <div>{project_location}</div>
    <input type="button" class="form-control" id="scanLocation" value="{project_location_button_label}" onclick={openProjectLocation}>
</div>
<div class="mb-3 f">
    <label for="projectDescription" class="form-label">Description</label>
    <textarea id="projectDescription" name="project_description" class="form-control" aria-describedby="projectDescriptionHelp" bind:value={project_description}></textarea>
    <div id="projectDescriptionHelp" class="form-text">A more detailed description about this project.</div>
</div>
<button type="submit" class="btn btn-primary" onclick={submit}>Create</button>
<button type="submit" class="btn btn-secondary" onclick={closeWindow}>Close</button>
</form>
</div>
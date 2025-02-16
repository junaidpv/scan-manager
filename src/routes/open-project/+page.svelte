<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from 'svelte'

    let projects: ProjectItem[] = $state([]);
    let success = $state(false);
    // To keep error message.
    let error_message = $state('Getting projects ...');
    
    onMount(async () => {
        try {
            let response_string: string = await invoke("get_projects");
            let response = JSON.parse(response_string)
            console.log(response);
            if (response.result) {
                projects = response.projects;
                success = true;
            }
            else {
                error_message = 'Something went wrong';
            }
        }catch(error) {
            error_message = 'Something went wrong';
        }
    })
</script>
<div class="container-fluid">
    {#if projects.length != 0}
    <div class="row">
        {#each projects as project}
            <div class="col col-md-6 col-lg-4">
                <div class="card">
                    <div class="card-body">
                        <h5 class="card-title">{project.name}</h5>
                        <dl class="card-text">
                            <dt>Location</dt><dd>{project.scan_location}</dd>
                            <dt>Description</dt><dd>{project.description}</dd>
                        </dl>
                        <button type="button" class="btn btn-primary">Open</button>
                    </div>
                </div>
            </div>
        {/each}
    </div>
    {/if}
    {#if !success}
        <p>{error_message}</p>
    {/if}
</div>
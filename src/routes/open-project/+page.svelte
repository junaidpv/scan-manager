<script>
    import { invoke } from "@tauri-apps/api/core";
    import {onMount} from 'svelte'

    let projects = $state();
    let success = $state(false);
    let error_message = $state('Getting projects ...');

    // projects = invoke("get_projects");
    
    onMount(async () => {
        try {
            let response_string = await invoke("get_projects");
            let response = JSON.parse(response_string)
            console.log(response);
            if (response.result) {
                projects = response.names;
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
    {#each projects as project_name}
        <button>{project_name}</button>
    {/each}
    {#if !success}
        <p>{error_message}</p>
    {/if}
</div>
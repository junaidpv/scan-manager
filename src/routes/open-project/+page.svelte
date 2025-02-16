<script>
    import { invoke } from "@tauri-apps/api/core";
    import {onMount} from 'svelte'

    let projects = $state();

    // projects = invoke("get_projects");
    
    onMount(async () => {
        try {
            let response_string = await invoke("get_projects");
            let response = JSON.parse(response_string)
            console.log(response);
            if (response.result) {
                projects = response.names;
            }
            else {
                // error = true;
                // error_message = response.message;
            }
        }catch(error) {
            console.error(error)
        }
    })
</script>
<div class="container-fluid">
    {#each projects as project_name}
        <p>{project_name}</p>
    {/each}
</div>
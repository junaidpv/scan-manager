<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Sidebar from "../lib/Sidebar.svelte";
  import LayoutDefault from "$lib/LayoutDefault.svelte";
  import ProjectInfo from "$lib/ProjectInfo.svelte";
  import { emit, listen } from '@tauri-apps/api/event'
  import Thumbnails from "$lib/Thumbnails.svelte";
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { openWindow } from "$lib/common";
    import Steps from "$lib/Steps.svelte";
    import StepTools from "$lib/StepTools.svelte";

  let project: ProjectItem | null = $state(null);
  
  let page_url = $state('/main');

  let active_image: string | null = $state(null);
  let thumbnails: ThumbnailItem[] = $state([]);
  let active_step = $state('fix_orientation');

  function disableParentWindow() {
    document.body.style.pointerEvents = 'none';
  }

  function enableParentWindow() {
    document.body.style.pointerEvents = 'auto';
  }

  let first: SidebarItemInfo = {
    title: 'Scan',
    description: 'Scan the project folder for scans',
    button_color: 'primary',
    icon: 'search',
    onclick: () => {
      page_url = '/misc?name=Junaid';
    }
  };
  let second: SidebarItemInfo = {
    title: 'Two',
    description: 'A two description',
    icon: 'fire',
    button_color: 'danger',
    onclick: () => {
      page_url = '/misc?name=Wonderful';
    }
  }
  let sidebar_items = [ first, second];

  async function get_project_images(project: ProjectItem) {
    let response_string = await invoke("get_project_images", {projectName: project.name}) as string;
    let response = JSON.parse(response_string);
    console.log(response);
    if (response.result) {
      thumbnails = [];
        // thumbnails = response.images;
        response.images.forEach((image: string) => {
            thumbnails.push({
                src: image
            });
        });
    }
    else {
        thumbnails = [];
    }
  }

  // listen to the `click` event and get a function to remove the event listener
  // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
  const unlisten = listen('project-selected', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    project = (event.payload as { projectData: ProjectItem }).projectData;
    get_project_images(project);
  });

  const listen_image_selected = listen('image-selected', (event) => {
    active_image = (event.payload as { src: string }).src;
  });

  listen('step_selected', (event) => {
    active_step = (event.payload as { step: string }).step;
  });

  let create_project_window: ChildWindow = {
    name: 'create-project',
    title: 'Create Project',
    url: '/create-project',
    close_callback: enableParentWindow
  };

  let open_project_window: ChildWindow = {
    name: 'open-project',
    title: 'Open Project',
    url: '/open-project',
    close_callback: enableParentWindow
  };

  // @ts-ignore
  async function onCreateProjectMenuClick(event) {
    openWindow(create_project_window, disableParentWindow);
  }

  // @ts-ignore
  async function onOpenProjectMenuClick(event) {
    openWindow(open_project_window, disableParentWindow);
  }

</script>

<main class="container-fluid">
  <LayoutDefault>
    {#snippet menu()}
      <div class="dropdown">
        <button class="btn btn-secondary dropdown-toggle" type="button" data-bs-toggle="dropdown" aria-expanded="false">
            Menu
        </button>
        <ul class="dropdown-menu">
            <li><button class="dropdown-item" onclick={onCreateProjectMenuClick}>Create Project ...</button></li>
            <li><button class="dropdown-item" onclick="{onOpenProjectMenuClick}">Open Project ...</button></li>
            <li><button class="dropdown-item" >Exit</button></li>
        </ul>
      </div>
    {/snippet}
    {#snippet project_info()}
      {#if project}
        <ProjectInfo project={project} />
      {/if}
    {/snippet}
    {#snippet first_sidebar()}
      <Sidebar items={sidebar_items} />
      <Steps />
      <StepTools step={active_step}/>
    {/snippet}
    {#snippet  content()}
      <!-- <iframe src={page_url} title="Title"></iframe> -->
      {#if active_image}
        <img src={convertFileSrc(active_image)} alt="Active" />
      {/if}
    {/snippet}
    {#snippet second_sidebar()}
      <Thumbnails thumbnails={thumbnails} />
    {/snippet}  
  </LayoutDefault>
</main>

<style>

</style>

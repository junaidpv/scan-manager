<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import LayoutDefault from "$lib/LayoutDefault.svelte";
  import ProjectInfo from "$lib/ProjectInfo.svelte";
  import { emit, listen } from '@tauri-apps/api/event'
  import Thumbnails from "$lib/Thumbnails.svelte";
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { openWindow } from "$lib/common";
    import Steps from "$lib/Steps.svelte";
    import StepTools from "$lib/StepTools.svelte";
    import GridOverlay from "$lib/GridOverlay.svelte";
    import CropOverlay from "$lib/CropOverlay.svelte";
    import { path } from "@tauri-apps/api";

  let project: ProjectItem | null = $state(null);
  
  let page_url = $state('/main');

  let active_image: ImageInfo | null = $state(null);
  let thumbnails: ThumbnailItem[] = $state([]);
  let image_infos: ImageInfo[] = $state([]);
  let active_step = $state('fix_orientation');
  let active_step_settings = $state({
    deskew_mode: 'manual',
    deskew_value: 0,
    frame_ratio_width: 1,
    frame_ratio_height: 1,
    rotate_angle: 0,
    frame_settings: {
      start_x: 0,
      start_y: 0,
      end_x: null,
      end_y: null,
    }
  });
  let grid_size = $state(20);

  function disableParentWindow() {
    document.body.style.pointerEvents = 'none';
  }

  function enableParentWindow() {
    document.body.style.pointerEvents = 'auto';
  }

  async function get_project_images(project: ProjectItem) {
    let response_string = await invoke("get_project_images", {projectName: project.name}) as string;
    let response = JSON.parse(response_string);
    console.log(response);
    if (response.result) {
      image_infos = [];
        // thumbnails = response.images;
        response.images.forEach((image_info: ImageInfo) => {
            image_infos.push(image_info);
        });
    }
    else {
        image_infos = [];
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
    active_image = (event.payload as { image_info: ImageInfo }).image_info;
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
            Project
        </button>
        <ul class="dropdown-menu">
            <li><button class="dropdown-item" onclick={onCreateProjectMenuClick}>Create ...</button></li>
            <li><button class="dropdown-item" onclick="{onOpenProjectMenuClick}">Open ...</button></li>
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
      <Steps />
      <StepTools step={active_step} bind:step_settings={active_step_settings}/>
    {/snippet}
    {#snippet  content()}
      <!-- <iframe src={page_url} title="Title"></iframe> -->
      {#if active_image}
        <div style="height: 100%; justify-content: center; align-items: center; position: relative;">
          <div class="image-wrapper" style="width: {active_image.width}px; height: {active_image.height}px; transform: rotate({active_step_settings.rotate_angle}deg) skew({active_step_settings.deskew_value}deg); position: relative;">
            <img width={active_image.width} height={active_image.height} src={convertFileSrc(active_image.path)} alt="Active" />
            <CropOverlay image_width={active_image.width} image_height={active_image.height} />
          </div>
          <GridOverlay grid_size={grid_size}  />
        </div>
      {/if}
    {/snippet}
    {#snippet second_sidebar()}
      <Thumbnails image_infos={image_infos} />
    {/snippet}  
  </LayoutDefault>
</main>

<style>

</style>

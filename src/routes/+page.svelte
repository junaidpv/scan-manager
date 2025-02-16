<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Sidebar from "../lib/Sidebar.svelte";
  import LayoutDefault from "$lib/LayoutDefault.svelte";
  import ProjectInfo from "$lib/ProjectInfo.svelte";
  import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
  import { emit, listen } from '@tauri-apps/api/event'
  import Thumbnails from "$lib/Thumbnails.svelte";
  import { convertFileSrc } from '@tauri-apps/api/core';

  let project: ProjectItem | null = $state(null);
  
  let page_url = $state('/main');

  function disableParentWindow() {
    document.body.style.pointerEvents = 'none';
  }

  function enableParentWindow() {
    document.body.style.pointerEvents = 'auto';
  }

  function createProjectWindow() {
    const webview = new WebviewWindow('create-project', {
      url: '/create-project',
    });

    disableParentWindow();
    
    // since the webview window is created asynchronously,
    // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
    webview.once('tauri://created', function (e) {
      console.log('success', e);
      
    })
    webview.once('tauri://error', function (e) {
      console.log(e);
    })
    webview.once('tauri://closed', function (e) {
      enableParentWindow();
    })
    webview.once("tauri://close-requested", function (e) {
      enableParentWindow();
    })
  }

  // @ts-ignore
  async function onCreateProjectMenuClick(event) {
    createProjectWindow();
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

  // listen to the `click` event and get a function to remove the event listener
  // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
  const unlisten = listen('project-selected', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    project = (event.payload as { projectData: ProjectItem }).projectData;
  });

  function openProjectWindow() {
    const webview = new WebviewWindow('open-project', {
      url: '/open-project',
    });

    disableParentWindow();
    
    // since the webview window is created asynchronously,
    // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
    webview.once('tauri://created', function (e) {
      console.log('success', e);
    })
    webview.once('tauri://error', function (e) {
      console.log(e);
    })
    webview.once('tauri://closed', function (e) {
      enableParentWindow();
    })
    webview.once("tauri://close-requested", function (e) {
      enableParentWindow();
    })
  }

  // @ts-ignore
  async function onOpenProjectMenuClick(event) {
    openProjectWindow();
  }

  let thumbnails = [
    {
      src: convertFileSrc("/media/data/project-files/indic-archive/Test Project/lycion.JPG"),
      alt: "Capsules",
    }
  ]

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
    {#snippet sidebar()}
      <Sidebar items={sidebar_items} />
    {/snippet}
    {#snippet  content()}
      <iframe src={page_url} title="Title"></iframe>
      <Thumbnails thumbnails={thumbnails} />
    {/snippet}
  </LayoutDefault>
</main>

<style>

</style>

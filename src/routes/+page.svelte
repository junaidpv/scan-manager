<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Sidebar from "../lib/Sidebar.svelte";
  import LayoutDefault from "$lib/LayoutDefault.svelte";
  import ProjectInfo from "$lib/ProjectInfo.svelte";
  import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
  import { emit, listen } from '@tauri-apps/api/event'
    import SidebarItem from "$lib/SidebarItem.svelte";

  let name = $state("");
  let greetMsg = $state("");
  let page = $state("main");

  let project: ProjectItem | null = $state(null);
  
  let page_url = $state('/main');

  function createProjectWindow() {
    const webview = new WebviewWindow('create-project', {
      url: '/create-project',
    });
    
    // since the webview window is created asynchronously,
    // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
    webview.once('tauri://created', function (e) {
      console.log('success', e);
    })
    webview.once('tauri://error', function (e) {
      console.log(e);
    })
    webview.once('tauri://closed', function (e) {
      console.log(e);
    })
  }

  // @ts-ignore
  async function onCreateProjectMenuClick(event) {
    createProjectWindow();
  }

  // @ts-ignore
  async function greet(event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("get_project_home_directory", { name });
    if (page == 'main') {
      page = 'misc';
      page_url = '/misc?name=Wonderful';
    }
    else {
      page = 'main';
      page_url = '/main';
    }
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
    
    // since the webview window is created asynchronously,
    // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
    webview.once('tauri://created', function (e) {
      console.log('success', e);
    })
    webview.once('tauri://error', function (e) {
      console.log(e);
    })
    webview.once('tauri://closed', function (e) {
      console.log(e);
    })
  }

  // @ts-ignore
  async function onOpenProjectMenuClick(event) {
    openProjectWindow();
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
    {#snippet sidebar()}
      <Sidebar items={sidebar_items} />
    {/snippet}
    {#snippet  content()}
    <iframe src={page_url} title="Title"></iframe>
    <h1>Welcome to Tauri + Svelte</h1>
    <img src="/bootstrap-icons/bootstrap.svg" alt="Bootstrap" width="32" height="32">
  
    <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>
  
    <form class="row" onsubmit={greet}>
      <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
      <button type="submit">Greet <i class="bi bi-chat"></i></button>
    </form>
    <p>{greetMsg}</p>
    {/snippet}
  </LayoutDefault>
</main>

<style>

</style>

<script>
  import { invoke } from "@tauri-apps/api/core";
  import Sidebar from "../lib/Sidebar.svelte";
  import LayoutDefault from "$lib/LayoutDefault.svelte";
  import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

  let name = $state("");
  let greetMsg = $state("");
  let page = $state("main");
  
  let page_url = $state('/main');

  function openWindow() {
    console.log('I am here');
    const webview = new WebviewWindow('create-project', {
      url: '/create-project',
    });
    
    // since the webview window is created asynchronously,
    // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
    webview.once('tauri://created', function () {
      console.log('success');
    })
    webview.once('tauri://error', function (e) {
      console.log(e);
    })
  }

  // @ts-ignore
  async function greet(event) {
    openWindow();
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
  let sidebar_items = [
    {
      title: 'One',
      description: 'A one description',
      button_color: 'warning',
      icon: null,
    },
    {
      title: 'Two',
      description: 'A two description',
      icon: 'fire',
      button_color: 'danger',
    }
  ];

</script>

<main class="container-fluid">
  <LayoutDefault>
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

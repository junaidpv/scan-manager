<script lang="ts">
    import Button from "./Button.svelte";
    let info: SidebarItemInfo = $props();
</script>
<div class="card">
    <Button title={info.title} description={info.description} icon={info.icon} button_color={info.button_color}/>
</div>
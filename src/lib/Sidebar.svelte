<script lang="ts">
    import SidebarItem from "./SidebarItem.svelte";

    interface Props {
        items: Array<SidebarItemInfo>;
    }
    let { items }: Props = $props();
</script>
<div>
    {#each items as item}
        <SidebarItem {...item}/>
    {/each}
</div>
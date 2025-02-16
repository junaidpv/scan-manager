<script lang="ts">
    import SidebarItem from "./SidebarItem.svelte";
    interface SidebarItemInfo {
		title: string;
        description: string;
        button_color: string;
        icon: string | null;
	}

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
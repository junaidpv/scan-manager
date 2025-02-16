<script lang="ts">
    let {title = 'Unkown', description = '', icon, button_color} = $props();

    let button_color_calculated = $state();

    $effect(() => {
		if (button_color) {
            button_color_calculated = 'text-' + button_color;
        }
	});
</script>
<button title={description}><div>{ title }</div>{#if icon }<i class="bi bi-{icon} fs-1 { button_color_calculated }"></i>{/if}</button>
<script>
    let {title = 'Unkown', description = '', icon = 'bootstrap', button_color} = $props();

    let button_color_calculated = $state();

    $effect(() => {
		if (button_color) {
            button_color_calculated = 'text-' + button_color;
        }
	});
</script>
<div class="card">
    <button title={description}><div>{ title }</div><i class="bi bi-{icon} fs-1 { button_color_calculated }"></i></button>
</div>
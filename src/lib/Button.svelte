<script lang="ts">
    let {title = 'Unkown', description = '', icon, button_color, onclick = null} = $props();

    let button_color_calculated = $state();

    $effect(() => {
		if (button_color) {
            button_color_calculated = 'text-' + button_color;
        }
	});
  function clicked(this: HTMLButtonElement, event: MouseEvent) {
    if (onclick) {
      onclick(event, this);
    }
  }
</script>
<button title={description} onclick={clicked}><div>{ title }</div>{#if icon }<i class="bi bi-{icon} fs-1 { button_color_calculated }"></i>{/if}</button>
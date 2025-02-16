<script>
    import Button from "./Button.svelte";
    let {title = 'Unkown', description = '', icon, button_color} = $props();
</script>
<div class="card">
    <Button title={title} description={description} icon={icon} button_color={button_color}/>
</div>
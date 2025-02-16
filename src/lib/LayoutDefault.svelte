<script>
    let {sidebar, content, menu, project_info} = $props();
</script>
<div class="row">
    <header class="col-12">
        {@render menu()}
        {@render project_info()}
    </header>
    <div class="col-2">{@render sidebar()}</div>
    <div class="col-10">{@render content()}</div>
</div>
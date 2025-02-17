<script>
    let {first_sidebar, second_sidebar, content, menu, project_info} = $props();
</script>
<div class="row">
    <header class="col-12">
        <div class="row">
            <div class="col-2">{@render menu()}</div>
            <div class="col-10">{@render project_info()}</div>
        </div>
    </header>
    <div class="col-2 limit-height">{@render first_sidebar()}</div>
    <div class="col-9 limit-height">{@render content()}</div>
    <div class="col-1 limit-height"><div>{@render second_sidebar()}</div></div>
</div>
<style>
    .limit-height {
        max-height: 95vh;
        overflow-y: auto;
    }
</style>
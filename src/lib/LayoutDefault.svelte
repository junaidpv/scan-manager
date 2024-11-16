<script>
    let {sidebar, content} = $props();
</script>
<div class="row">
    <div class="col-2">{@render sidebar()}</div>
    <div class="col-10">{@render content()}</div>
</div>
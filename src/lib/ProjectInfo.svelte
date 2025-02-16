<script lang="ts">
    export let project: ProjectItem;
</script>
<div>
    <span><strong>Project Name</strong>{project.name}</span>
    <span><strong>Scan Location</strong>{project.scan_location}</span>
    <span><strong>Description</strong>{project.description}</span>
    <span><strong>Created At</strong>{new Date(project.created_at).toLocaleString()}</span>
    <span><strong>Updated At</strong>{new Date(project.updated_at).toLocaleString()}</span>
</div>
<script lang="ts">
    export let project: ProjectItem;
</script>
<div>
    <div><strong>Project Name</strong>{project.name}</div>
    <div><strong>Scan Location</strong>{project.scan_location}</div>
    <div><strong>Description</strong>{project.description}</div>
    <div><strong>Created At</strong>{new Date(project.created_at).toLocaleString()}</div>
    <div><strong>Updated At</strong>{new Date(project.updated_at).toLocaleString()}</div>
</div>
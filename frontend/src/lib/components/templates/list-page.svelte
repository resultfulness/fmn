<script>
const { children } = $props();
</script>

<div class="page">
    {@render children()}
</div>

<style>
.page {
    padding: 1rem;
    overflow-y: auto;
    flex: 1;
}
</style>

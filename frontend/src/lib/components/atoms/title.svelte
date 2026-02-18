<script>
const { children } = $props();
</script>

<h1>{@render children()}</h1>

<style>
h1 {
    font: var(--font-title);
    margin: 0;
    text-align: center;
}
</style>

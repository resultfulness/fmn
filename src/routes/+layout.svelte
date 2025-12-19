<script lang="ts">
import Header from "$lib/components/header.svelte";
import Footer from "$lib/components/footer.svelte";
import "../app.css";

let { children } = $props();
</script>

<div class="app">
    <Header />
    <main>
        {@render children()}
    </main>
    <Footer />
</div>

<style>
.app {
    width: min(800px, 100%);
    height: 100vh;
    height: 100svh;
    margin-inline: auto;
    display: flex;
    flex-direction: column;
}

main {
    position: relative;
    flex: 1;
    overflow-y: hidden;
    display: flex;
    flex-direction: column;
}
</style>

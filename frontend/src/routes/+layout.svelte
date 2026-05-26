<script lang="ts">
import "../app.css";
import "$lib/styles/typography.css";
import { onNavigate } from "$app/navigation";
import Confirm from "$lib/ui/confirm.svelte";
import Toast from "$lib/ui/toast.svelte";
import Header from "$lib/ui/header.svelte";
import Footer from "$lib/ui/footer.svelte";

let { children } = $props();
onNavigate(navigation => {
    if (!document.startViewTransition) return;

    return new Promise(resolve => {
        document.startViewTransition(async () => {
            resolve();
            await navigation.complete;
        });
    });
});
</script>

<Confirm />
<Toast />
<div class="app">
    <Header initialTitle="forget-me-not" />
    <main>
        {@render children()}
    </main>
    <Footer />
</div>

<style>
.app {
    width: min(800px, 100%);
    height: 100vh;
    height: 100dvh;
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

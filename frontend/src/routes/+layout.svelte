<script lang="ts">
import "../app.css";
import { onNavigate } from "$app/navigation";
import Confirm from "$lib/components/confirm.svelte";
import Toast from "$lib/components/toast.svelte";
import Header from "$lib/components/organisms/header.svelte";
import Footer from "$lib/components/organisms/footer.svelte";

let { children } = $props();
onNavigate((navigation) => {
    if (!document.startViewTransition) return;

    return new Promise((resolve) => {
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

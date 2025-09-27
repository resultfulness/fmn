<script>
import { toastStore } from "$lib/stores/toast-store.svelte";
</script>

carts!

<button onclick={() => {
    toastStore.show("lol", "uhoh", 5000);
    toastStore.show("lol2", "yayy", 2500);
}}>send</button>

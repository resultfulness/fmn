<script module lang="ts">
type HeaderState = {
    title: string;
    backUrl?: string;
};

export const HeaderState = $state<HeaderState>({
    title: "forget me not",
});
</script>

<script lang="ts">
import { beforeNavigate } from "$app/navigation";
import { ArrowLeft } from "@lucide/svelte";
import IconButton from "./icon-button.svelte";

const { title, backUrl } = $derived(HeaderState);

beforeNavigate(() => {
    HeaderState.title = "forget me not";
    HeaderState.backUrl = undefined;
});
</script>

<header>
    <div>
        {#if backUrl}
            <IconButton
                variant="secondary"
                href={backUrl}
                icon={ArrowLeft}
                size={32}
            />
        {/if}
    </div>
    <h1>{title}</h1>
    <div></div>
</header>

<style>
header {
    display: grid;
    grid-template-columns: 48px 1fr 48px;
    align-items: center;
    gap: 0.25rem;
    text-align: center;
    padding: 0.5rem;
}

h1 {
    font: var(--font-header);
    margin: 0;
}
</style>

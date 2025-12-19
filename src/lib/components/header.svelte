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
import Button from "./button.svelte";

const { title, backUrl } = $derived(HeaderState);

beforeNavigate(() => {
    HeaderState.title = "forget me not";
    HeaderState.backUrl = undefined;
})
</script>

<header>
    <div>
        {#if backUrl}
            <Button square variant="empty" href={backUrl}>
                <ArrowLeft size={30} />
            </Button>
        {/if}
    </div>
    <h1>{title}</h1>
    <div></div>
</header>

<style>
header {
    padding-inline: 1rem;
    display: grid;
    grid-template-columns: 32px 1fr 32px;
    align-items: center;
    gap: 0.25rem;
    text-align: center;
}
</style>

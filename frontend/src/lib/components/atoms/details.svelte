<script lang="ts">
import type { Snippet } from "svelte";

interface DetailsProps {
    summary: string;
    children: Snippet;
    open: boolean;
}

const { summary, children, open }: DetailsProps = $props();

let details: HTMLDetailsElement = $state()!;
let summaryElement: HTMLElement = $state()!;
let content: HTMLDivElement = $state()!;

let animation: Animation | null = $state(null);
let animationState: "expanding" | "closing" | "eeping" = $state("eeping");

function onclick(e: MouseEvent) {
    e.preventDefault();
    details.style.overflow = "hidden";
    if (animationState == "closing" || !details.open) {
        details.style.height = `${details.offsetHeight}px`;
        details.open = true;
        window.requestAnimationFrame(() => {
            animationState = "expanding";
            createAnimation(true);
        });
    } else if (animationState == "expanding" || details.open) {
        animationState = "closing";
        createAnimation(false);
    }
}

function createAnimation(opening: boolean) {
    const start = `${details.offsetHeight}px`;
    const end = `${
        summaryElement.offsetHeight + (opening ? content.offsetHeight : 0)
    }px`;

    if (animation) {
        animation.cancel();
    }

    animation = details.animate(
        { height: [start, end] },
        {
            duration: 250,
            easing: window
                .getComputedStyle(document.documentElement)
                .getPropertyValue("--easing"),
        }
    );

    animation.onfinish = () => {
        details.open = opening;
        animation = null;
        animationState = "eeping";
        details.style.height = details.style.overflow = "";
    };
    animation.oncancel = () => (animationState = "eeping");
}
</script>

<details {open} bind:this={details}>
    <summary class="text-emph" bind:this={summaryElement} {onclick}>
        {summary}
    </summary>
    <div class="content" bind:this={content}>
        {@render children()}
    </div>
</details>

<style>
details {
    margin-block: 0.5rem;
}

summary {
    padding: 0.75rem;
    background-color: var(--clr-overlay);
    border-radius: var(--rounding);
    box-shadow: var(--shadow);
}

.content {
    padding-block: 0.5rem;
}
</style>

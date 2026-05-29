import { navigating } from "$app/state";
import { setLastRecipe } from "$lib/last-recipe.svelte";
import type { PageLoad } from "./$types";

export const load: PageLoad = () => {
    if (navigating.from && navigating.from.route.id === "/recipes/[id]") {
        setLastRecipe(-1);
    }
};

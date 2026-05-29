import { error } from "@sveltejs/kit";
import type { LayoutLoad } from "./$types";
import api from "$lib/api";
import { setLastRecipe } from "$lib/last-recipe.svelte";
import request from "$lib/api/request";

export const load: LayoutLoad = async ({ params, fetch }) => {
    if (!Number.isInteger(+params.id)) {
        error(422, `recipe id '${params.id}' not a number`);
    }
    const id = +params.id;

    request.fetch = fetch;
    const recipe = await api.recipes.read(id);
    request.fetch = window.fetch;
    if (!recipe) {
        error(404, `no recipe with id ${id}`);
    }

    setLastRecipe(id);

    return { recipe };
};

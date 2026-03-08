import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import api from "$lib/api";

export const load: PageLoad = async ({ params }) => {
    if (!Number.isInteger(+params.id)) {
        error(422, `recipe id '${params.id}' not a number`);
    }
    const id = +params.id;

    const recipe = await api.recipes.read(id);
    if (!recipe) {
        error(404, `no recipe with id ${id}`);
    }

    return { recipe };
};

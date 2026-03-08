import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import api from "$lib/api";

export const load: PageLoad = async ({ params }) => {
    if (!Number.isInteger(+params.id)) {
        error(422, `item id '${params.id}' not a number`);
    }
    const id = +params.id;

    const item = await api.items.read(id);
    if (!item) {
        error(404, `no item with id ${id}`);
    }

    return { item };
}

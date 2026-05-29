import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import api from "$lib/api";
import request from "$lib/api/request";

export const load: PageLoad = async ({ params, fetch }) => {
    if (!Number.isInteger(+params.id)) {
        error(422, `item id '${params.id}' not a number`);
    }
    const id = +params.id;

    request.fetch = fetch;
    const item = await api.items.read(id);
    request.fetch = window.fetch;
    if (!item) {
        error(404, `no item with id ${id}`);
    }

    return { item };
};

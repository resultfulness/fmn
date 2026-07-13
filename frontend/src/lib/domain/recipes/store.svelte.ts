import api from "$lib/api";
import type { Recipe } from "./recipe";

const recipeStore = $state({
    recipes: <Recipe[]>[],
    async load() {
        this.recipes = await api.recipes.readAll();
    },
});

export default recipeStore;

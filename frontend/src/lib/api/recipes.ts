import { Recipe, RecipeShort } from "$lib/schemas/recipes";

let lsRecipes = localStorage.getItem("recipes");
let recipes = JSON.parse(lsRecipes ?? "[]") as Recipe[];
let maxId = recipes.length;

export default {
    readAll(): Promise<RecipeShort[]> {
        return new Promise(res =>
            setTimeout(
                () => res(recipes.map(recipe => RecipeShort.parse(recipe))),
                100
            )
        );
    },
};

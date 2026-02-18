import { Recipe, RecipeShort } from "$lib/schemas/recipes";

let lsRecipes = localStorage.getItem("recipes");
const recipes = {
    recipes: JSON.parse(lsRecipes ?? "[]") as Recipe[],
    maxId: 0,
    readAll(): Promise<RecipeShort[]> {
        return new Promise(res =>
            setTimeout(
                () =>
                    res(this.recipes.map(recipe => RecipeShort.parse(recipe))),
                100
            )
        );
    },
};

export default recipes;

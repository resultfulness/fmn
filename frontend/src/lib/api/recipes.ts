import { Recipe, RecipeCreate, RecipeUpdate } from "$lib/domain/recipes/recipe";
import request from "./request";

export default {
    async create(recipe: RecipeCreate): Promise<Recipe> {
        return await request.post("/recipes", recipe);
    },
    async read(id: number): Promise<Recipe> {
        return await request.get(`/recipes/${id}`);
    },
    async readAll(): Promise<Recipe[]> {
        return await request.get("/recipes");
    },
    async update(id: number, recipe: RecipeUpdate): Promise<Recipe> {
        return await request.patch(`/recipes/${id}`, recipe);
    },
    async delete(id: number): Promise<Recipe> {
        return await request.delete(`/recipes/${id}`);
    },
};

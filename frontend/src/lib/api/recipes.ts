import { Recipe, RecipeCreate, RecipeUpdate } from "$lib/schemas/recipes";
import request from "./request";

export default {
    create(recipe: RecipeCreate): Promise<Recipe> {
        return request.post("/recipes", recipe);
    },
    read(id: number): Promise<Recipe> {
        return request.get(`/recipes/${id}`);
    },
    readAll(): Promise<Recipe[]> {
        return request.get("/recipes");
    },
    update(id: number, recipe: RecipeUpdate): Promise<Recipe> {
        return request.patch(`/recipes/${id}`, recipe);
    },
    delete(id: number): Promise<Recipe> {
        return request.delete(`/recipes/${id}`);
    },
};

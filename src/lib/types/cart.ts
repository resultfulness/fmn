import type { Item } from "./item";

export type CartItem = {
    origin: string;
} & Item;

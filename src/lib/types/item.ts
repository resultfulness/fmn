export type Item = {
    item_id: number;
    name: string;
    icon: string;
};

export type Items = {
    items: Item[],
    count: number,
}

export type ItemAdd = {
    name: string;
    icon: string;
}

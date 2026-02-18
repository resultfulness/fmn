import { Item, ItemShort, ItemUpdate, type ItemCreate } from "$lib/schemas/items";

let lsItems = localStorage.getItem("items");
const items = {
    items: JSON.parse(lsItems ?? "[]") as Item[],
    maxId: 0,
    create(item: ItemCreate): Promise<ItemShort> {
        const newItem = <Item>{ ...item, item_id: ++this.maxId };
        this.items.push(newItem);
        localStorage.setItem("items", JSON.stringify(this.items));
        return new Promise(res => res(ItemShort.parse(newItem)));
    },
    read(id: number): Promise<ItemShort | undefined> {
        return new Promise(res =>
            res(this.items.find(({ item_id }) => item_id === id))
        );
    },
    readAll(): Promise<ItemShort[]> {
        return new Promise(res => setTimeout(() => res(
            this.items.map(item => ItemShort.parse(item))
        ), 100));
    },
    update(id: number, item: ItemUpdate): Promise<ItemShort> {
        return new Promise(res => {
            const i = this.items.findIndex(item => item.item_id === id);
            const updatedItem = <Item>{ ...this.items[i], ...item };
            this.items[i] = updatedItem;
            localStorage.setItem("items", JSON.stringify(this.items));
            res(ItemShort.parse(updatedItem));
        });
    },
    delete(id: number): Promise<ItemShort> {
        return new Promise(res => {
            const item = this.items.find(item => item.item_id === id);
            this.items = this.items.filter(item => item.item_id !== id);
            localStorage.setItem("items", JSON.stringify(this.items));
            res(ItemShort.parse(item));
        });
    }
}

export default items;

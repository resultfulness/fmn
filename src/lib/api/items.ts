import { Item, ItemShort, ItemUpdate, type ItemCreate } from "$lib/schemas/items";

const items = {
    items: [] as Item[],
    maxId: 0,
    create(item: ItemCreate): Promise<ItemShort> {
        const newItem = <Item>{ ...item, item_id: ++this.maxId };
        this.items.push(newItem)
        return new Promise(res => res(ItemShort.parse(newItem)));
    },
    readAll(): Promise<ItemShort[]> {
        return new Promise(res => setTimeout(() => res(
            this.items.map(item => ItemShort.parse(item))
        ), 1000));
    },
    update(id: number, item: ItemUpdate): Promise<ItemShort> {
        return new Promise(res => {
            const i = this.items.findIndex(item => item.item_id === id);
            const updatedItem = <Item>{ ...this.items[i], ...item };
            this.items[i] = updatedItem;
            res(ItemShort.parse(updatedItem));
        });
    },
    delete(id: number): Promise<ItemShort> {
        return new Promise(res => {
            const item = this.items.find(item => item.item_id === id);
            this.items = this.items.filter(item => item.item_id !== id);
            res(ItemShort.parse(item));
        });
    }
}

export default items;

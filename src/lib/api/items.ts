import type { Item } from "$lib/types";

const itemsApi = {
    getAll() {
        return new Promise<Item[]>(res => {
            setTimeout(() => {
                res([{
                    id: 1,
                    name: "burger",
                    icon: "",
                }]);
            }, 2000)
        });
    }
};

export default itemsApi;

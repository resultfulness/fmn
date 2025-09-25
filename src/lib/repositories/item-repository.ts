import api from "$lib/api";

const itemRepository = {
    async getAll() {
        return await api.items.getAll();
    }
}

export default itemRepository;

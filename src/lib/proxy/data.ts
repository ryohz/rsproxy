import { ModesInFeatures, type TabObject } from "../types";
import History from "./history/History.svelte";

const all: ModesInFeatures[] = [
    new ModesInFeatures({
        name: "History",
        iconify: "material-symbols:history",
        element: History
    }),
]

function tab_objects(): TabObject[] {
    let objects: TabObject[] = []
    for (let i = 0; i < all.length; i++) {
        objects.push(all[i].for_tabs());
    }
    return objects;
}

export const modes = {
    all: all,
    tab_objects: tab_objects,
}
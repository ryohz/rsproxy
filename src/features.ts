import Home from "./lib/home/Home.svelte";
import Proxy from "./lib/proxy/Proxy.svelte";
import { Feature, type SidebarObject } from "./lib/types";

const all: Feature[] = [
    new Feature({
        name: "Home",
        iconify: "bxs:home",
        is_feature: false,
        element: Home
    }),
    new Feature({
        name: "Proxy",
        iconify: "simple-icons:traefikproxy",
        is_feature: true,
        element: Proxy
    })
]

function sidebar_objects(): SidebarObject[] {
    let objects: SidebarObject[] = []
    for (let i = 0; i < all.length; i++) {
        let object = all[i].sidebar();
        objects.push(object);
    }
    return objects;
}

function names(): string[] {
    let names: string[] = [];
    for (let i = 0; i < all.length; i++) {
        names.push(all[i].name);
    }
    return names;
}

export const features = {
    all: all,
    sidebar_objects: sidebar_objects,
    names: names
}
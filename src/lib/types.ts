import { get } from "svelte/store";
import { all_exchanges_count } from "./datas";
import type { SvelteComponent } from "svelte";

export interface SidebarObject {
    name: string,
    iconify: string,
    // if is_feature is true, the icon will be recognized as a part of features like "proxy", "ehco"...
    is_feature: boolean,
}

export class Feature {
    name: string;
    iconify: string;
    is_feature: boolean;
    element: any;

    constructor(arg: { name: string, iconify: string, is_feature: boolean, element: any }) {
        this.name = arg.name;
        this.iconify = arg.iconify;
        this.is_feature = arg.is_feature;
        this.element = arg.element;
    }

    sidebar(): SidebarObject {
        return {
            name: this.name,
            iconify: this.iconify,
            is_feature: this.is_feature
        }
    }
}

export class RelativePosition {
    top: number | undefined;
    bottom: number | undefined;
    left: number | undefined;
    right: number | undefined;

    constructor(top: number | undefined, bottom: number | undefined, left: number | undefined, right: number | undefined) {
        this.top = top;
        this.bottom = bottom;
        this.left = left;
        this.right = right;
    }

    parse(): string {
        let position = "position: absolute;"
        let h_is_indicated = false;
        let v_is_indicated = false;

        if (this.top !== undefined) {
            position += `top: ${this.top}px;`;
            v_is_indicated = true;
        }
        if (this.bottom !== undefined && !h_is_indicated) {
            position += `bottom: ${this.bottom}px;`;
            v_is_indicated = true;
        }

        if (this.left !== undefined) {
            position += `left: ${this.left}px;`;
            h_is_indicated = true;
        }
        if (this.right !== undefined && !v_is_indicated) {
            position += `right: ${this.right}px;`;
            h_is_indicated = true;
        }

        if (!v_is_indicated) {
            position += `top: 0px`;
            v_is_indicated = true;
        }
        if (!h_is_indicated) {
            position += `left: 0px;`;
            h_is_indicated = true;
        }

        return position;
    }
}

export interface TabObject {
    name: string,
    iconify: string,
}

export class ModesInFeatures {
    name: string;
    iconify: string;
    element: any;

    constructor(args: { name: string, iconify: string, element: any }) {
        this.name = args.name;
        this.iconify = args.iconify;
        this.element = args.element;
    }

    for_tabs(): TabObject {
        return {
            name: this.name,
            iconify: this.iconify
        }
    }
}

export enum ExchangeType {
    Request,
    Response,
    Empty
}

export interface Part {
    id: number,
    type: ExchangeType,
    headers: object,
    url: string,
    status: number | undefined,
    method: string | undefined
}

export class Exchange {
    id: number;
    type: ExchangeType;
    headers: object;
    body: string;
    url: string;
    status: number | undefined;
    method: string | undefined;

    constructor(args: { headers: object, body: string, url: string, method: string | undefined, status: number | undefined, type: ExchangeType }) {
        all_exchanges_count.update((n) => n + 1);
        this.id = get(all_exchanges_count);
        this.type = args.type;
        this.headers = args.headers;
        this.body = args.body;
        this.url = args.url;
        this.method = args.method;
        this.status = args.status;
    }

    part(): Part {
        return {
            id: this.id,
            type: this.type,
            headers: this.headers,
            url: this.url,
            status: this.status,
            method: this.method,
        }
    }
}

export interface Component {
    component: any,
    props: any,
}

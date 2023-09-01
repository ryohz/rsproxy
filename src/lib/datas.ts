import { writable, type Writable } from "svelte/store";
import { Exchange, ExchangeType } from "./types";

export const all_exchanges: Writable<Exchange[]> = writable([]);
export const history_exchanges: Writable<Exchange[]> = writable([]);
export const all_exchanges_count = writable(0);
export const current_history_exchange: Writable<Exchange> = writable();


export enum HttpMethod {
    GET,
    POST,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE
}
export function parse_http_method(_method: string): HttpMethod | undefined {
    let method = _method.toUpperCase();
    switch (method) {
        case HttpMethod.GET.toString():
            return HttpMethod.GET;
        case HttpMethod.POST.toString():
            return HttpMethod.POST;
        case HttpMethod.DELETE.toString():
            return HttpMethod.DELETE;
        case HttpMethod.HEAD.toString():
            return HttpMethod.HEAD;
        case HttpMethod.OPTIONS.toString():
            return HttpMethod.OPTIONS;
        case HttpMethod.PATCH.toString():
            return HttpMethod.PATCH;
        case HttpMethod.TRACE.toString():
            return HttpMethod.TRACE;
        default:
            return undefined;
    }
}





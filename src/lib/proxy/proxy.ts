import { listen } from "@tauri-apps/api/event";
import { Exchange, ExchangeType } from "../types";
import { get, writable } from "svelte/store";

const exchanges = writable<Exchange[]>([]);

let request_unlisten;
let response_unlisten;

async function start() {
    request_unlisten = await listen("proxy_request", (event) => {
        let request = JSON.parse(event.payload);
        let body = request.body;
        let headers = request.headers;
        let new_exchanges = get(exchanges);
        new_exchanges.push({
            headers: headers,
            body: body,
            type: ExchangeType.Request
        })
        exchanges.set(new_exchanges);
    });

    response_unlisten = await listen("proxy_response", (event) => {
        let response = JSON.parse(event.payload);
        let body = response.body;
        let headers = response.headers;
        let new_exchanges = get(exchanges);
        new_exchanges.push({
            headers: headers,
            body: body,
            type: ExchangeType.Response
        })
        exchanges.set(new_exchanges);
    });
}

export const proxy = {
    start: start,
    exchanges: exchanges,
}
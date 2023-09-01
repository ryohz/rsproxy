import { listen } from "@tauri-apps/api/event";
import { Exchange, ExchangeType, type Part } from "../types";
import { get, writable } from "svelte/store";
import { history_exchanges } from "../datas";

let request_unlisten;
let response_unlisten;

async function start() {
    request_unlisten = await listen("proxy_request", (event) => {
        let request = JSON.parse(event.payload);
        if (typeof request.headers === "string" &&
            typeof request.body === "string" &&
            typeof request.url === "string" &&
            typeof request.method === "string") {
            let headers: string = request.headers;
            let body: string = request.body;
            let url: string = request.url;
            let method: string = request.method;

            let new_exchanges = get(history_exchanges);

            new_exchanges.push(new Exchange({
                headers: headers,
                body: body,
                url: url,
                method: method,
                status: undefined,
                type: ExchangeType.Request,
            }))
            history_exchanges.set(new_exchanges);
        }
    });

    response_unlisten = await listen("proxy_response", (event) => {
        let response = JSON.parse(event.payload);
        if (typeof response.headers === "string" &&
            typeof response.body === "string" &&
            typeof response.url === "string" &&
            typeof response.status === "number") {
            let headers: object = response.headers;
            let body: string = response.body;
            let url: string = response.url;
            let status: number = response.status;

            let new_exchanges = get(history_exchanges);

            new_exchanges.push(new Exchange({
                headers: headers,
                body: body,
                url: url,
                method: undefined,
                status: status,
                type: ExchangeType.Response,
            }))

            history_exchanges.set(new_exchanges);
        }
    });
}

export const proxy = {
    start: start,
}

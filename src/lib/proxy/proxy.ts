import { emit, listen } from "@tauri-apps/api/event";
import { get } from "svelte/store";
import { writable, type Writable } from "svelte/store";

let request_unlisten;
let response_unlisten;

const all_exchanges: Writable<Exchange[]> = writable([]);
const history_exchanges: Writable<Exchange[]> = writable([]);
const all_exchanges_count = writable(0);
const current_history_exchange: Writable<Exchange> = writable();

enum HttpMethod {
    GET,
    POST,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE
}

function parse_http_method(_method: string): HttpMethod | undefined {
    let method = _method.toUpperCase();
    switch (method) {
        case "GET":
            return HttpMethod.GET;
        case "POST":
            return HttpMethod.POST;
        case "DELETE":
            return HttpMethod.DELETE;
        case "HEAD":
            return HttpMethod.HEAD;
        case "OPTIONS":
            return HttpMethod.OPTIONS;
        case "PATCH":
            return HttpMethod.PATCH;
        case "TRACE":
            return HttpMethod.TRACE;
        default:
            return undefined;
    }
}


async function start() {
    request_unlisten = await listen("proxy_request", (event) => {
        let request = JSON.parse(event.payload);
        if (typeof request.headers === "string" && typeof request.body === "string" && typeof request.url === "string" && typeof request.method === "string") {
            console.log(request.piloted);
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
                type: ExchangeType.Request
            }));
            history_exchanges.set(new_exchanges);
        }
    });

    response_unlisten = await listen("proxy_response", (event) => {
        let response = JSON.parse(event.payload);
        if (typeof response.headers === "string" && typeof response.body === "string" && typeof response.url === "string" && typeof response.status === "number") {
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
                type: ExchangeType.Response
            }));

            history_exchanges.set(new_exchanges);
        }
    });
}

let pilot_state = writable(false);

async function enable_pilot() {
    pilot_state.set(true);
    await send_pilot_state();
}

async function disable_pilot() {
    pilot_state.set(false);
    await send_pilot_state();
}

async function send_pilot_state() {
    emit("change_pilot_state", get(pilot_state));
}



export enum ExchangeType {
    Request,
    Response,
    Empty
}


export class Exchange {
    id: number;
    type: ExchangeType;
    headers: string;
    body: string;
    url: string;
    status: number | undefined;
    method: HttpMethod | undefined;

    constructor(args: { headers: string, body: string, url: string, method: string | undefined, status: number | undefined, type: ExchangeType }) {
        proxy.all_exchanges_count.update((n) => n + 1);
        if (args.method !== undefined) {
            let method: HttpMethod | undefined = parse_http_method(args.method);

            this.id = get(proxy.all_exchanges_count);
            this.type = args.type;
            this.headers = args.headers;
            this.body = args.body;
            this.url = args.url;
            this.method = method;
            this.status = args.status;
        } else {
            this.id = get(proxy.all_exchanges_count);
            this.type = args.type;
            this.headers = args.headers;
            this.body = args.body;
            this.url = args.url;
            this.method = args.method;
            this.status = args.status;
        }
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

function get_http_methods_string(method: HttpMethod | undefined): string {
    if (method === undefined) {
        return "ERROR";
    }
    switch (method) {
        case HttpMethod.GET:
            return "GET"
        case HttpMethod.DELETE:
            return "DELETE"
        case HttpMethod.HEAD:
            return "HEAD"
        case HttpMethod.OPTIONS:
            return "OPTIONS"
        case HttpMethod.PATCH:
            return "PATCH"
        case HttpMethod.POST:
            return "POST"
        case HttpMethod.TRACE:
            return "TRACE"
    }
}

// wtf is following code?
function get_http_methods(): HttpMethod[] {
    return [HttpMethod.GET, HttpMethod.POST, HttpMethod.DELETE, HttpMethod.HEAD, HttpMethod.OPTIONS, HttpMethod.PATCH, HttpMethod.TRACE]
}

export interface Part {
    id: number,
    type: ExchangeType,
    headers: string,
    url: string,
    status: number | undefined,
    method: HttpMethod | undefined
}



export const proxy = {
    start: start,
    pilot_state: pilot_state,
    enable_pilot: enable_pilot,
    disable_pilot: disable_pilot,
    all_exchanges: all_exchanges,
    history_exchanges: history_exchanges,
    all_exchanges_count: all_exchanges_count,
    current_history_exchange: current_history_exchange,
    parse_http_method: parse_http_method,
    HttpMethod: HttpMethod,
    ExchangeType: ExchangeType,
    get_http_methods_string: get_http_methods_string,
    get_http_methods: get_http_methods
};

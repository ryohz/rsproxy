import { get, writable, type Writable } from "svelte/store";
import { empty_request, empty_response, Request, Response, type RustRequest, type RustResponse } from "../../exchange";
import { listen } from "@tauri-apps/api/event";

// ** proxy_start function will be called by main.ts at beginning of frontend.
// ** since that, it will store all request and response to following variables.

export const request_history: Writable<Request[]> = writable([]);
export const response_history: Writable<Response[]> = writable([]);

// ** a function to find a response that is pair with the given request.
// ** pair_id is shared between a request and a response of the same transaction.

export function find_response(id: string): Response {
    let responses = get(response_history);
    let resp = responses.find(rs => rs.pair_id === id)
    if (resp !== undefined) {
        return resp;
    } else {
        return empty_response();
    }
}




export let current_request: Writable<Request> = writable();
export let current_response: Writable<Response> = writable();

current_request.subscribe(req => {
    current_response.set(find_response(req?.pair_id));
});

response_history.subscribe(() => {
    if (get(current_response) === undefined && get(current_request) !== undefined) {
        current_response.set(find_response(get(current_request).pair_id));
    }
});

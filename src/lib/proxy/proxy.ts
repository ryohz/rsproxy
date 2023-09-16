import { emit, listen } from "@tauri-apps/api/event";
import { get } from "svelte/store";
import { writable, type Writable } from "svelte/store";
import { capitalize } from "../common";
import { type RustRequest, type RustResponse, Request, Response } from "../exchange";

export const request_history: Writable<Request[]> = writable([]);
export const response_history: Writable<Response[]> = writable([]);

export async function proxy_start() {
    await listen<string>("proxy-request", (e) => {
        let rq: RustRequest = JSON.parse(e.payload);
        let new_rq = new Request(rq);
        request_history.update(rqs => {
            rqs.push(new_rq);
            return rqs
        });
    });

    await listen<string>("proxy-response", (e) => {
        let rs: RustResponse = JSON.parse(e.payload);
        let new_rs = new Response(rs);
        response_history.update(rss => {
            rss.push(new_rs);
            return rss;
        });
    });
}

request_history.subscribe(() => {
});
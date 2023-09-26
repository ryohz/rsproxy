import { emit, listen } from "@tauri-apps/api/event";
import { get } from "svelte/store";
import { writable, type Writable } from "svelte/store";
import { type RustRequest, type RustResponse, Request, Response, empty_response } from "../exchange";
import { request_history, response_history } from "./history/history";

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


export const pilot_state = writable(false);
export const pilot_exchange_list: Writable<(Request | Response)[]> = writable([]);

request_history.subscribe(() => {
    // ** pilot state management
    if (get(pilot_state)) {
        console.log("hello");
        let reqs = get(request_history);
        pilot_exchange_list.update(list => {
            let item = reqs[reqs.length - 1];
            if (item !== undefined) {
                list.push(item);
            }
            return list;
        });
    }
});

response_history.subscribe(() => {
    // ** pilot state management
    if (get(pilot_state)) {
        let ress = get(response_history);
        pilot_exchange_list.update(list => {
            let item = ress[ress.length - 1];
            if (item !== undefined) {
                list.push(ress[ress.length - 1])
            }
            return list;
        });
    }
});

pilot_state.subscribe(() => {
    emit("pilot-state", get(pilot_state));
});




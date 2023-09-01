import { writable, type Writable } from "svelte/store";
import { Exchange, ExchangeType } from "./types";

export const all_exchanges: Writable<Exchange[]> = writable([]);
export const history_exchanges: Writable<Exchange[]> = writable([]);
export const all_exchanges_count = writable(0);
export const current_history_exchange: Writable<Exchange> = writable();

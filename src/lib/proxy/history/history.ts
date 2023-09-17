import { writable, type Writable } from "svelte/store";
import { empty_request, Request } from "../../exchange";

export let current_request: Writable<Request> = writable(empty_request());
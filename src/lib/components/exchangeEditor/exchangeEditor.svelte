<script lang="ts">
    import { get, writable, type Writable } from "svelte/store";
    import { Response, type Request } from "../../exchange";
    import Tabs from "../tabs/Tabs.svelte";
    import "./exchangeEditor.css";
    import AnExchangeEditor from "../anExchangeEditor/anExchangeEditor.svelte";

    export let request: Writable<Request | undefined>;
    export let response: Writable<Response | undefined>;

    let request_content = writable("");
    let response_content = writable("");

    let req = get(request);
    if (req !== undefined) {
        request_content.set(req.to_editable());
    }
    let res = get(response) as Response;
    response_content.set(res.to_editable());

    request.subscribe((req) => {
        if (req !== undefined) {
            request_content.set(req.to_editable());
        }
    });

    response.subscribe((res) => {
        response_content.set((res as Response).to_editable());
    });

    let items: { name: string; icon: string }[] = [
        { name: "request", icon: "" },
        { name: "response", icon: "" },
    ];
    let current = "request";
    function update(next: string) {
        current = next;
    }
</script>

<div class="editor">
    <Tabs {items} {current} {update} />
    {#if current === "request"}
        <AnExchangeEditor content={request_content} />
    {:else if current === "response"}
        <AnExchangeEditor content={response_content} />
    {/if}
</div>

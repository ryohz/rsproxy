<script lang="ts">
    import { get, writable, type Writable } from "svelte/store";
    import {
        empty_request,
        empty_response,
        Response,
        type Request,
    } from "../../exchange";
    import { find_response, response_history } from "../../proxy/proxy";
    import Tabs from "../tabs/Tabs.svelte";
    import "./exchangeEditor.css";

    export let request: Request;
    export let response: Response;

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
        {#if !request.is_empty}
            <pre>
            {request.to_editable()}
        </pre>
            <!-- {:else} -->
        {/if}
    {:else if current === "response"}
        {#if !response.is_empty}
            <pre>
            {response.to_editable()}
        </pre>
            <!-- {:else} -->
        {/if}
    {/if}
</div>

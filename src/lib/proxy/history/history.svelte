<script lang="ts">
    import "./history.css";
    import Box from "../../components/box/box.svelte";
    import { empty_request, Response, type Request } from "../../exchange";
    import { get, writable, type Writable } from "svelte/store";
    import { find_response, request_history } from "../proxy";
    import ExchangeEditor from "../../components/exchangeEditor/exchangeEditor.svelte";
    import { current_request } from "./history";
    import { afterUpdate } from "svelte";

    let requests: Request[] = get(request_history);
    request_history.subscribe(() => {
        requests = get(request_history);
    });

    let current = get(current_request);

    function update(rq: Request) {
        current_request.set(rq);
        current = rq;
        console.log(find_response(current.pair_id));
    }
    
</script>

<div class="history">
    <Box>
        <div class="list" slot="top">
            {#each requests as rq}
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <div class="exchange" on:click={() => update(rq)}>
                    <p>{rq.method}</p>
                    <p>{rq.url}</p>
                </div>
            {/each}
        </div>
        <ExchangeEditor
            request={current}
            response={find_response(current.pair_id)}
            slot="bottom"
        />
    </Box>
</div>

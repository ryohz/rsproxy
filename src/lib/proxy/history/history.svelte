<script lang="ts">
    import "./history.css";
    import Box from "../../components/box/box.svelte";
    import { type Request } from "../../exchange";
    import { get } from "svelte/store";
    import { request_history } from "./history";
    import ExchangeEditor from "../../components/exchangeEditor/exchangeEditor.svelte";
    import { current_request, current_response } from "./history";

    let requests: Request[] = get(request_history);
    request_history.subscribe(() => {
        requests = get(request_history);
    });

    function update(rq: Request) {
        current_request.set(rq);
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
            request={current_request}
            response={current_response}
            slot="bottom"
        />
    </Box>
</div>

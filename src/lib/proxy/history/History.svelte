<script lang="ts">
    import "./scss/history.css";

    import Box from "../../components/box/Box.svelte";
    import ExchangeEditor from "../../components/exchangeEditor/ExchangeEditor.svelte";
    import { get } from "svelte/store";
    import List from "./List.svelte";
    import type { Exchange } from "../../types";
    import { current_history_exchange } from "../../datas";
    import { afterUpdate } from "svelte";

    export let current_exchange: Exchange = get(current_history_exchange);

    current_history_exchange.subscribe(() => {
        current_exchange = get(current_history_exchange);
    });
</script>

<div class="history">
    <Box
        components={[
            { component: List, props: undefined },
            {
                component: ExchangeEditor,
                props: {
                    current_exchange: current_exchange,
                },
            },
        ]}
    />
</div>

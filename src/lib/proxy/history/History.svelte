<script lang="ts">
    import "./scss/history.css";
    import Box from "../../components/box/Box.svelte";
    import ExchangeEditor from "../../components/exchangeEditor/ExchangeEditor.svelte";
    import { get } from "svelte/store";
    import List from "./List.svelte";
    import { Exchange, proxy } from "../proxy";

    export let current_exchange: Exchange = get(proxy.current_history_exchange);

    proxy.current_history_exchange.subscribe(() => {
        current_exchange = get(proxy.current_history_exchange);
    });
</script>

<div class="history">
    <Box
        components={[
            { component: List, props: undefined },
            {
                component: ExchangeEditor,
                props: {
                    exchange: current_exchange,
                },
            },
        ]}
    />
</div>

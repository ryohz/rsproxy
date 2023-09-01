<script lang="ts">
    import { proxy } from "../proxy";
    import { get } from "svelte/store";
    import { ExchangeType } from "../../types";
    import "./scss/list.css";
    import ExchangePart from "../../components/exchangePart/ExchangePart.svelte";
    import { history_exchanges } from "../../datas";

    export let exchanges = get(history_exchanges);
    history_exchanges.subscribe(() => {
        exchanges = get(history_exchanges);
    });
</script>

<div class="list">
    {#each exchanges as exchange}
        <ExchangePart part={exchange.part()} />
    {/each}
</div>

<script lang="ts">
    import { proxy } from "../proxy";
    import { get } from "svelte/store";
    import "./scss/list.css";
    import ExchangePart from "../../components/exchangePart/ExchangePart.svelte";

    export let exchanges = get(proxy.history_exchanges);
    proxy.history_exchanges.subscribe(() => {
        exchanges = get(proxy.history_exchanges);
    });
</script>

<div class="list">
    {#each exchanges as exchange}
        <ExchangePart part={exchange.part()} />
    {/each}
</div>

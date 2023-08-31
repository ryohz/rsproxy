<script lang="ts">
    import { proxy } from "../proxy";
    import { get } from "svelte/store";
    import { ExchangeType } from "../../types";
    import "./scss/viewer.css";

    export let exchanges = get(proxy.exchanges);
    proxy.exchanges.subscribe(() => {
        exchanges = get(proxy.exchanges);
        console.log("hello");
    });
</script>

<div class="viewer">
    {#each exchanges as exchange}
        {#if exchange.type === ExchangeType.Request}
            <!-- <div class="exchange request" /> -->
            <p>{exchange.headers}</p>
        {:else if exchange.type === ExchangeType.Response}
            <!-- <div class="exchange request" /> -->
            <p>{exchange.headers}</p>
        {/if}
    {/each}
</div>

<script lang="ts">
    import "./exchangePart.css";
    import {
        ExchangeType,
        get_http_methods_string,
        type Part,
    } from "../../types";
    import { current_history_exchange, history_exchanges } from "../../datas";
    import { get } from "svelte/store";

    export let part: Part;

    function handle(id: number) {
        let exchanges = get(history_exchanges);
        for (let i = 0; i < exchanges.length; i++) {
            if (exchanges[i].id === id) {
                current_history_exchange.set(exchanges[i]);
            }
        }
    }
</script>

{#if part.type === ExchangeType.Request}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
        class="exchange request"
        on:click={() => {
            handle(part.id);
        }}
    >
        <p>{get_http_methods_string(part.method)}</p>
        <p>{part.url}</p>
    </div>
{:else if part.type === ExchangeType.Response}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
        class="exchange response"
        on:click={() => {
            handle(part.id);
        }}
    >
        <p>{part.status}</p>
        <p>{part.url}</p>
    </div>
{/if}

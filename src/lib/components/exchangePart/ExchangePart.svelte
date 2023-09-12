<script lang="ts">
    import "./exchangePart.css";
    import { proxy, type Part } from "../../proxy/proxy";
    import { get } from "svelte/store";

    export let part: Part;

    function handle(id: number) {
        let exchanges = get(proxy.history_exchanges);
        for (let i = 0; i < exchanges.length; i++) {
            if (exchanges[i].id === id) {
                proxy.current_history_exchange.set(exchanges[i]);
            }
        }
    }
</script>

{#if part.type === proxy.ExchangeType.Request}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
        class="exchange request"
        on:click={() => {
            handle(part.id);
        }}
    >
        <p>{proxy.get_http_methods_string(part.method)}</p>
        <p>{part.url}</p>
    </div>
{:else if part.type === proxy.ExchangeType.Response}
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

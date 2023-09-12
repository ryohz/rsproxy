<script lang="ts">
    import ExchangeEditor from "../../components/exchangeEditor/ExchangeEditor.svelte";
    import Controller from "./Controller.svelte";
    import "./scss/pilot.css";
    import { Exchange, proxy } from "../proxy";
    import { get } from "svelte/store";

    let exchange = get(proxy.pilot_exchange);

    proxy.pilot_exchange.subscribe(() => {
        exchange = get(proxy.pilot_exchange);
    });
</script>

<div class="pilot">
    <Controller />
    <ExchangeEditor {exchange} />
</div>

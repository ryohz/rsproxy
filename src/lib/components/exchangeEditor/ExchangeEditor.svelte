<script lang="ts">
    import "./exchangeEditor.css";
    import { Exchange, ExchangeType } from "../../types";

    export let current_exchange: Exchange = new Exchange({
        headers: {},
        body: "",
        url: "",
        method: undefined,
        status: undefined,
        type: ExchangeType.Empty,
    });

    console.log(current_exchange);
</script>

<div class="editor">
    <p class="body">
        {current_exchange.body}
    </p>
</div>

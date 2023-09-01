<script lang="ts">
    import "./exchangeEditor.css";
    import {
        Exchange,
        ExchangeType,
        get_http_methods,
        get_http_methods_string,
    } from "../../types";
    import Select from "../select/Select.svelte";
    import TextInput from "../textInput/TextInput.svelte";
    import HeadersEditor from "./editor/Editor.svelte";

    export let exchange: Exchange = new Exchange({
        headers: "",
        body: "",
        url: "",
        method: undefined,
        status: undefined,
        type: ExchangeType.Empty,
    });

    let methods = get_http_methods();
    let methods_string: string[] = [];
    for (const method of methods) {
        methods_string.push(get_http_methods_string(method));
    }
</script>

<div class="editor">
    <div class="header">
        {#if exchange.type === ExchangeType.Request}
            <div class="method">
                <p>method</p>
                <Select
                    value={get_http_methods_string(exchange.method)}
                    items={methods_string}
                />
            </div>
        {:else if exchange.type === ExchangeType.Response}
            <div class="status">
                <p>Status</p>
                <TextInput value={exchange.status?.toString()} />
            </div>
        {:else}{/if}
        <div class="url">
            <p>url</p>
            <TextInput value={exchange.url} />
        </div>
    </div>
    <div class="headers">
        <HeadersEditor body={exchange.body} headers={exchange.headers}/>
    </div>
</div>

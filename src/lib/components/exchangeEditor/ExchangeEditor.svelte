<script lang="ts">
    import "./exchangeEditor.css";
    import { proxy, Exchange, ExchangeType } from "../../proxy/proxy";
    import Select from "../select/Select.svelte";
    import TextInput from "../textInput/TextInput.svelte";
    import HeadersEditor from "./editor/Editor.svelte";
    import Button from "../switch/Switch.svelte";
    import Icon from "@iconify/svelte";
    import Editor from "./editor/Editor.svelte";

    export let exchange: Exchange = new Exchange({
        headers: "",
        body: "",
        url: "",
        method: undefined,
        status: undefined,
        type: ExchangeType.Empty,
    });

    let methods = proxy.get_http_methods();
    let methods_string: string[] = [];
    for (const method of methods) {
        methods_string.push(proxy.get_http_methods_string(method));
    }
</script>

{#if exchange.type === ExchangeType.Request || exchange.type === ExchangeType.Response}
    <div class="editor">
        <div class="header">
            {#if exchange.type === ExchangeType.Request}
                <div class="method">
                    <p>Method</p>
                    <Select
                        value={proxy.get_http_methods_string(exchange.method)}
                        items={methods_string}
                    />
                </div>
                <div class="url">
                    <p>Url</p>
                    <TextInput value={exchange.url} />
                </div>
            {:else if exchange.type === ExchangeType.Response}
                <div class="status">
                    <p>Status</p>
                    <TextInput value={exchange.status?.toString()} />
                </div>
                <div class="url">
                    <p>Url</p>
                    <TextInput value={exchange.url} />
                </div>
            {/if}
        </div>
        <div class="main">
            <Editor body={exchange.body} headers={exchange.headers} />
            <div class="side" />
        </div>
    </div>
{:else}
    <div class="editor">
        <div class="empty">
            <Icon icon="fa-regular:sad-cry" class="icon" />
            <p class="empty-message">
                It haven't recieved any requests or responses yet
            </p>
        </div>
    </div>
{/if}

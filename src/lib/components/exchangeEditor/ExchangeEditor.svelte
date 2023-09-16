<script lang="ts">
    import "./exchangeEditor.css";
    import { proxy, Exchange, ExchangeType } from "../../proxy/proxy";
    import Select from "../select/Select.svelte";
    import TextInput from "../textInput/TextInput.svelte";
    import Icon from "@iconify/svelte";
    import Tabs from "../tabs/Tabs.svelte";
    import type { TabObject } from "../../types";

    export let Pair: Exchange[];

    let tab_objects: TabObject[] = [
        {
            name: "request",
            iconify: "",
        },
        {
            name: "response",
            iconify: "",
        },
    ];
    let current_mode = "request";
    function change_mode(name: string) {
        current_mode = name;
    }
</script>

<div class="editor">
    <Tabs {tab_objects} {current_mode} {change_mode} />
</div>

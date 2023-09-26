<script lang="ts">
    import "./pilot.css";
    import { get, writable, type Writable } from "svelte/store";
    import Button from "../../components/button/button.svelte";
    import Switch from "../../components/switch/switch.svelte";
    import { pilot_exchange_list, pilot_state } from "../proxy";
    import AnExchangeEditor from "../../components/anExchangeEditor/anExchangeEditor.svelte";
    import { to_request } from "../../exchange";

    let current_content: Writable<string> = writable("");
    pilot_exchange_list.subscribe(() => {
        let list = get(pilot_exchange_list);
        let current_exchange = list[0];
        current_content.set(current_exchange?.to_editable());
    });

    function forward() {
        to_request(get(current_content));
    }
</script>

<div class="pilot">
    <div class="top">
        <div class="btn">
            <Switch value={["enable", "disable"]} state={pilot_state} />
            <Button value="foward" on_click={forward} />
        </div>
    </div>
    <AnExchangeEditor content={current_content} />
</div>

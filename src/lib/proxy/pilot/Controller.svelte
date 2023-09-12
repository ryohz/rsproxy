<script lang="ts">
    import "./scss/controller.css";
    import Button from "../../components/button/Button.svelte";
    import Switch from "../../components/switch/Switch.svelte";
    import { proxy } from "../proxy";
    import { get } from "svelte/store";

    let pilot_state = get(proxy.pilot_state);
    proxy.pilot_state.subscribe(() => {
        pilot_state = get(proxy.pilot_state);
    });

    function change_pilot_state() {
        if (pilot_state) {
            proxy.disable_pilot();
        } else {
            proxy.enable_pilot();
        }
    }

    function forward() {}

    function discard() {}
</script>

<div class="controller">
    <div class="inner">
        <Switch
            value="OFF"
            after_value="ON"
            on_click={change_pilot_state}
            current_value={pilot_state ? "ON" : "OFF"}
        />
        <Button iconify="icon-park-outline:delete" on_click={discard} />
        <Button iconify="icon-park-outline:delete" on_click={discard} />
        <Button iconify="ph:play-fill" on_click={forward} />
    </div>
</div>

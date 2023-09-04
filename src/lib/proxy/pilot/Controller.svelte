<script lang="ts">
    import "./scss/controller.css";
    import Button from "../../components/button/Button.svelte";
    import Switch from "../../components/switch/Switch.svelte";
    import { emit } from "@tauri-apps/api/event";

    function change_pilot_state(state: boolean) {
        if (state) {
            emit('front-to-back', 'hello');
        } else {
        }
    }

    function forward() {}

    function discard() {}
</script>

<div class="controller">
    <Switch value="OFF" after_value="ON" on_click={change_pilot_state} />
    <Button iconify="icon-park-outline:delete" on_click={discard} />
    <Button iconify="ph:play-fill" on_click={forward} />
</div>

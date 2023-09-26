<script lang="ts">
    import { get, type Writable } from "svelte/store";
    import "./switch.css";

    export let value: string[];
    export let state: Writable<boolean>;

    let s = get(state);

    state.subscribe(() => {
        s = get(state);
    });

    function oc() {
        state.set(!get(state));
        s = get(state);
    }
</script>

<input
    type="button"
    class={`switch ${s ? "on" : "off"}`}
    value={s ? value[0] : value[1]}
    on:click={oc}
/>

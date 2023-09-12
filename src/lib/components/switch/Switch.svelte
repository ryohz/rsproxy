<script lang="ts">
    import "./switch.css";
    export let value: string;
    export let after_value: string;
    export let on_click: Function;

    export let current_value = value;

    function oc() {
        if (current_value === value) {
            current_value = after_value;
            on_click(true);
        } else if (current_value === after_value) {
            current_value = value;
            on_click(false);
        }
    }
</script>

<input
    type="button"
    class={`switch ${current_value === after_value ? "on" : "off"}`}
    value={current_value}
    on:click={oc}
/>

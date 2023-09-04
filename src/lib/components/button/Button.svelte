<script lang="ts">
    import Icon from "@iconify/svelte";
    import "./button.css";
    export let value: string = "";
    export let iconify: string = "";
    export let on_click: Function;
</script>

<button class="button" on:click={() => on_click()}>
    <Icon icon={iconify} />
    {value}
</button>

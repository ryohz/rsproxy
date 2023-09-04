<script lang="ts">
    import "./scss/proxy.css";
    import History from "./history/History.svelte";
    import { modes } from "./data";
    import Tabs from "../components/tabs/Tabs.svelte";

    let current_mode = modes.tab_objects()[0].name;

    function change_mode(name: string) {
        current_mode = name;
    }
</script>

<div class="proxy">
    <Tabs tab_objects={modes.tab_objects()} {current_mode} {change_mode} />
    {#each modes.all as mode}
        {#if mode.name === current_mode}
            <svelte:component this={mode.element} />
        {/if}
    {/each}
</div>

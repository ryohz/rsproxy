<script lang="ts">
    import type { TabObject } from "../../types";
    import Icon from "@iconify/svelte";
    import "./tabs.css";

    export let tab_objects: TabObject[];
    export let current_mode: string;
    export let change_mode: Function;
</script>

<div class="tabs">
    {#each tab_objects as tab_object}
        {#if tab_object.name === current_mode}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div
                class="tab selected"
                on:click={() => change_mode(tab_object.name)}
            >
                <Icon icon={tab_object.iconify} />
                <p class="name">
                    {tab_object.name}
                </p>
            </div>
        {:else}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div class="tab" on:click={() => change_mode(tab_object.name)}>
                <Icon icon={tab_object.iconify} />
                <p class="name">
                    {tab_object.name}
                </p>
            </div>
        {/if}
    {/each}
</div>

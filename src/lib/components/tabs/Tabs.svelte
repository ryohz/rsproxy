<script lang="ts">
    import type { TabObject } from "../../types";
    import Icon from "@iconify/svelte";
    import "./tabs.css";

    export let items: { name: string; icon: string }[];
    export let current: string;
    export let update: Function;
</script>

<div class="tabs">
    {#each items as item}
        {#if item.name === current}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div class="tab selected" on:click={() => update(item.name)}>
                <Icon icon={item.icon} />
                <p class="name">
                    {item.name}
                </p>
            </div>
        {:else}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div class="tab" on:click={() => update(item.name)}>
                <Icon icon={item.icon} />
                <p class="name">
                    {item.name}
                </p>
            </div>
        {/if}
    {/each}
</div>

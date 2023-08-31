<script lang="ts">
    import Icon from "@iconify/svelte";
    import "./sidebar.css";
    import { RelativePosition, type SidebarObject } from "../../types";
    import Caption from "../caption/Caption.svelte";

    export let objects: SidebarObject[];
    export let position: string;
    export let update: Function;
    export let current_mode: string;
</script>

<div class='bar'>
    <div class="icons others">
        {#each objects as object}
            {#if object.is_feature}
                {#if object.name === current_mode}
                    <!-- svelte-ignore a11y-no-static-element-interactions -->
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <span on:click={() => update(object.name)}>
                        <Icon
                            icon={object.iconify}
                            class="icon other selected"
                        />
                    </span>
                {:else}
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <!-- svelte-ignore a11y-no-static-element-interactions -->
                    <span on:click={() => update(object.name)}>
                        <Icon icon={object.iconify} class="icon other" />
                    </span>
                {/if}
            {/if}
        {/each}
    </div>
    <div class="icons features">
        {#each objects as object}
            {#if !object.is_feature}
                {#if object.name === current_mode}
                    <!-- svelte-ignore a11y-no-static-element-interactions -->
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <span on:click={() => update(object.name)}>
                        <Icon
                            icon={object.iconify}
                            class="icon feature selected"
                        />
                    </span>
                {:else}
                    <!-- svelte-ignore a11y-no-static-element-interactions -->
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <span on:click={() => update(object.name)}>
                        <Icon icon={object.iconify} class="icon feature" />
                    </span>
                {/if}
            {/if}
        {/each}
    </div>
</div>

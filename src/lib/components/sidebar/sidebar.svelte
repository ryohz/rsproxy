<script lang="ts">
    import Icon from "@iconify/svelte";
    import "./sidebar.css";

    export let items: {
        name: string;
        icon: string;
    }[];

    export let current: string;

    export let update: Function;
</script>

<div class="bar">
    <div class="icons others">
        {#each items as item}
            {#if item.name === current}
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <span on:click={() => update(item.name)}>
                    <Icon icon={item.icon} class="icon other selected" />
                </span>
            {:else}
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <span on:click={() => update(item.name)}>
                    <Icon icon={item.icon} class="icon other" />
                </span>
            {/if}
        {/each}
    </div>
</div>

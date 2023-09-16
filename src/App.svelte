<script lang="ts">
    import Sidebar from "./lib/components/sidebar/Sidebar.svelte";
    // import { features } from "./features";
    import "./scss/app.css";

    // let current_mode = features.names()[1];
    // function update_mode(mode: string) {
    //     current_mode = mode;
    // }
</script>

<svelte:head>
    <link rel="preconnect" href="https://rsms.me/" />
    <link rel="stylesheet" href="https://rsms.me/inter/inter.css" />
</svelte:head>

<!-- <body> -->
<div class="app">
    <Sidebar
        position="left"
        objects={features.sidebar_objects()}
        update={update_mode}
        {current_mode}
    />

    {#each features.all as feature}
        {#if feature.name === current_mode}
            <svelte:component this={feature.element} />
        {/if}
    {/each}
</div>
<!-- </body> -->

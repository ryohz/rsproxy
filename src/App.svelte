<script lang="ts">
    import Sidebar from "./lib/components/sidebar/sidebar.svelte";
    import Proxy from "./lib/proxy/proxy.svelte";
    import "./scss/app.css";

    let items: { name: string; icon: string }[] = [
        { name: "proxy", icon: "eos-icons:proxy" },
    ];

    let current = "proxy";

    function update(next: string) {
        current = next;
    }
</script>

<svelte:head>
    <link rel="preconnect" href="https://rsms.me/" />
    <link rel="stylesheet" href="https://rsms.me/inter/inter.css" />
</svelte:head>

<div class="app">
    <Sidebar {items} {current} {update} />
    {#if current === "proxy"}
        <Proxy />
    {/if}
</div>

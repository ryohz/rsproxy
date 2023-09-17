<script lang="ts">
    import "./proxy.css";
    import Tabs from "../components/tabs/Tabs.svelte";
    import History from "./history/history.svelte";

    let items: { name: string; icon: string }[] = [
        { name: "history", icon: "material-symbols:history" },
        { name: "pilot", icon: "material-symbols:history" },
    ];
    let current = "history";
    function update(next: string) {
        current = next;
    }
</script>

<div class="proxy">
    <Tabs {items} {current} {update} />
    {#if current === "history"}
        <History />
    {:else if current === "pilot"}{/if}
</div>

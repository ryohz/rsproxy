<script lang="ts">
    import { beforeUpdate } from "svelte";
    import "./select.css";

    export let value: string;
    export let items: string[];

    let options_state = false;

    function handle_option_state() {
        options_state = !options_state;
    }

    function handle_value_change(_value: string) {
        value = _value;
    }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
    class="select"
    on:mouseleave={() => {
        options_state = false;
    }}
>
    {#if options_state}
        <input
            type="button"
            class="button active-button"
            {value}
            on:click={handle_option_state}
        />
    {:else}
        <input
            type="button"
            class="button"
            {value}
            on:click={handle_option_state}
        />
    {/if}
    {#if options_state}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="options" on:click={handle_option_state}>
            {#each items as item}
                <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
                <p class="option" on:click={() => handle_value_change(item)}>
                    {item}
                </p>
            {/each}
        </div>
    {/if}
</div>

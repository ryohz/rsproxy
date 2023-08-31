<script lang="ts">
    import { get, writable } from "svelte/store";
    import "./box.css";

    export let components: any[];

    while (components.length !== 2) {
        components = components.pop();
    }

    // export let dif = 0;
    export let dif = 0;
    let origin: number;
    let state = false;

    let border_size = "5px";
    let top_size = `calc(50% - (${border_size} / 2) + ${
        dif.toString() + "px"
    })`;
    let bottom_size = `calc(50% - (${border_size} / 2) - ${
        dif.toString() + "px"
    })`;
    let grid_template_rows = `grid-template-rows: ${top_size} ${border_size} ${bottom_size};`;

    function init_orogin(event) {
        state = true;
        if (!origin) {
            origin = event.clientY;
        }
    }

    function resize(event) {
        if (state) {
            if (origin) {
                dif = event.clientY - origin;
            }
            top_size = `calc(50% - (${border_size} / 2) + ${
                dif.toString() + "px"
            })`;
            bottom_size = `calc(50% - (${border_size} / 2) - ${
                dif.toString() + "px"
            })`;
            grid_template_rows = `grid-template-rows: ${top_size} ${border_size} ${bottom_size};`;
        }
    }

    function stop_resize(event) {
        state = false;
        // origin += event.clientY;
    }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
    class="box"
    on:mouseup={stop_resize}
    on:mousemove={resize}
    style={grid_template_rows}
>
    {#each components as component}
        <svelte:component this={component} />
        {#if components[components.length - 1] !== component}
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div
                class="border"
                on:mousedown={init_orogin}
                on:mousemove={resize}
                on:mouseup={stop_resize}
            />
        {/if}
    {/each}
</div>

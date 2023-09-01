<script lang="ts">
    import "./box.css";
    import type { Component } from "../../types";
    import { afterUpdate } from "svelte";

    export let components: Component[];

    // export let dif = 0;
    export let dif = 0;
    let origin: number;
    let state = false;

    let border_size = "4px";
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
    }

    afterUpdate(() => {
        console.log(grid_template_rows);
    });
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- <div class='entire'> -->
<div
    class="box"
    on:mouseup={stop_resize}
    on:mousemove={resize}
    style={grid_template_rows}
>
    <svelte:component this={components[0].component} {...components[0].props} />
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
        class="border"
        on:mousedown={init_orogin}
        on:mousemove={resize}
        on:mouseup={stop_resize}
    />
    <svelte:component this={components[1].component} {...components[1].props} />
</div>
<!-- </div> -->

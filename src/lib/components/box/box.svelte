<script lang="ts">
    import "./box.css";

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

    function init_orogin(event) {
        document.body.style.cursor = "row-resize";
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
        }
    }

    function stop_resize(event) {
        state = false;
        document.body.style.cursor = "auto";
    }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="box" on:mouseup={stop_resize} on:mousemove={resize}>
    <div class="top content" style={`height: ${top_size};`}>
        <slot name="top" />
    </div>
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
        style={`top: ${top_size}; height: ${border_size};`}
        class="border"
        on:mousedown={init_orogin}
        on:mousemove={resize}
        on:mouseup={stop_resize}
    />

    <div class="bottom content" style={`height: ${bottom_size};`}>
        <slot name="bottom" />
    </div>
</div>

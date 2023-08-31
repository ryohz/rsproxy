<script lang="ts">
    import type { RelativePosition } from "../../types";

    export let name: string;
    export let position: RelativePosition;
</script>

<div style={position.parse()} class="caption">
    <p>
        {name}
    </p>
</div>

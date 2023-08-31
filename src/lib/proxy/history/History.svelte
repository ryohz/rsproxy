<script>
    import { listen } from "@tauri-apps/api/event";
    import "./scss/history.css";

    import Box from "../../components/box/Box.svelte";
    import Viewer from "./Viewer.svelte";
    import Dummy from "../../components/dummy/Dummy.svelte";

    let components = [Viewer, Dummy];
</script>

<div class="history">
    <Box {components} />
</div>

<script lang="ts">
    import { get, type Writable } from "svelte/store";
    import "./anExchangeEditor.css";

    export let content: Writable<string>;
    
    let c = get(content);
    content.subscribe(() => {
        c = get(content);
    });
</script>

<pre>
    {c}
</pre>

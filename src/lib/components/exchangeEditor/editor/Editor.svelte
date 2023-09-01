<script lang="ts">
    import "./editor.css";
    import * as prettier from "prettier";
    import pluginBabel from "prettier/plugins/babel";
    import pluginEstree from "prettier/plugins/estree";
    import pluginHtml from "prettier/plugins/html";
    import { onMount } from "svelte";

    export let headers: string;
    export let body: string;

    async function formatJSON(headers: string) {
        try {
            return await prettier.format(headers, {
                parser: "json",
                plugins: [pluginBabel, pluginEstree],
            });
        } catch (error) {
            return headers;
        }
    }

    function determine_format() {
        let json = JSON.parse(headers);
        let type = json["content-type"];
        
    }

    async function formatBody(text: string) {
        determine_format();
    }
</script>

<div class="headers">
    {#await formatJSON(headers) then formattedJSON}
        <textarea class="editor" value={formattedJSON} />
    {/await}
    {#await formatBody(body)}{/await}
</div>

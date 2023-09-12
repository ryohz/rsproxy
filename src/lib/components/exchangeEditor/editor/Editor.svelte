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

    async function formatBody(text: string) {
        let json = JSON.parse(headers);
        let type = json["content-type"];

        switch (type) {
            case "text/html":
                try {
                    return await prettier.format(text, {
                        parser: "babel",
                        plugins: [pluginBabel, pluginEstree, pluginHtml],
                    });
                } catch (error) {}
            case "text/css":
                try {
                    return await prettier.format(text, {
                        parser: "css",
                        plugins: [pluginBabel, pluginEstree],
                    });
                } catch (error) {}
            case "text/javascript":
                try {
                    return await prettier.format(text, {
                        parser: "javascript",
                        plugins: [pluginBabel, pluginEstree],
                    });
                } catch (error) {}
            case "application/json":
                try {
                    return await prettier.format(text, {
                        parser: "json",
                        plugins: [pluginBabel, pluginEstree],
                    });
                } catch (error) {}
            default:
                return text;
        }
    }
</script>

<div class="headers">
    {#await formatJSON(headers) then formattedJSON}
        <textarea class="editor" value={formattedJSON} />
    {/await}
    {#await formatBody(body) then formattedBody}
        <textarea class="editor" value={formattedBody} />
    {/await}
</div>

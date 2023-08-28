<script lang="ts">
    import { emit, listen } from "@tauri-apps/api/event";

    let unlisten1: any;
    let unlisten2: any;
    async function f() {
        unlisten1 = await listen("proxy_request", (event) => {
            console.log(event);
        });
        unlisten2 = await listen("proxy_response", (event) => {
            console.log(event);
        });
    }
    f();
</script>

<button>heelo</button>

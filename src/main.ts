import "./scss/styles.css";
import App from "./App.svelte";
import { proxy_start } from "./lib/proxy/proxy";

proxy_start();

const app = new App({
  target: document.getElementById("app"),
});

export default app;

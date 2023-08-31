import "./scss/styles.css";
import App from "./App.svelte";
import { proxy } from "./lib/proxy/proxy";

proxy.start();

const app = new App({
  target: document.getElementById("app"),
});

export default app;

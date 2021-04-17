<script lang="ts">
  import Router from "svelte-spa-router";
  import type { RouteDefinition } from "svelte-spa-router";

  import Login from "./pages/Login.svelte";

  import Main from "./pages/Main.svelte";

  const routes: RouteDefinition = {
    "/": Main,
    "/login": Login,
  };
</script>

<main>
  <Router {routes} />
</main>

<script>
  import Router from "svelte-spa-router";
  import Login from "./pages/Login.svelte";

  import Main from "./pages/Main.svelte";

  const routes = {
    "/": Main,
    "/login": Login,
  };
</script>

<main>
  <Router {routes} />
</main>

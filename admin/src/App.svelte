<script lang="ts">
  import { getLinks } from "./lib/api";

  const promise = getLinks();
</script>

<main>
  <div class="container">
    {#await promise}
      <p>...waiting</p>
    {:then links}
      <table class="table is-striped is-fullwidth is-hoverable">
        <thead>
          <tr>
            <th>Link</th>
            <th>Destination</th>
            <th>Description</th>
            <th>Public</th>
          </tr>
        </thead>
        <tbody>
          {#each links as link}
            <tr>
              <td>{link.name}</td>
              <td><a target="_blank" href={link.url}>{link.url}</a></td>
              <td>{link.description || ""}</td>
              <td>{link.public ? "Yes" : "No"}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {:catch error}
      <p style="color: red">{error.message}</p>
    {/await}
  </div>
</main>

<style>
  .container {
    margin-top: 30px;
  }
</style>

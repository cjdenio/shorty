<script lang="ts">
  import Icon from "svelte-fa";
  import {
    faTrashAlt,
    faPlus,
    faEdit,
  } from "@fortawesome/free-solid-svg-icons";
  import Modal from "../lib/Modal.svelte";

  import { deleteLink, getLinks } from "../lib/api";
  import type { Link } from "../lib/api";
  import { onMount } from "svelte";

  import { token } from "../lib/stores";
  import { push } from "svelte-spa-router";

  token.subscribe(async (i) => {
    console.log("yay");
    if (i == "") {
      await push("/login");
    }
  });

  let links: Link[] = [];
  let deleteModal: {
    active: boolean;
    link: string;
  } = {
    active: false,
    link: "",
  };

  onMount(async () => {
    links = await getLinks({});
  });

  function deleteLinkHandler(name: string) {
    deleteModal.link = name;
    deleteModal.active = true;
  }

  async function deleteLinkConfirm(name: string) {
    console.log(`deleting: ${name}`);
    await deleteLink({ name });
    links = await getLinks({});
  }
</script>

<main>
  <Modal
    bind:active={deleteModal.active}
    title="Delete?"
    content={`Are you sure you want to delete the link ${deleteModal.link}?`}
    style="danger"
    text="Delete"
    on:submit={() => deleteLinkConfirm(deleteModal.link)}
  />

  <div class="container">
    <div class="level">
      <div class="level-left">
        <span class="has-text-grey"
          >{links.length} link{links.length == 1 ? "" : "s"}</span
        >
      </div>
      <div class="level-right">
        <div class="level-item">
          <button class="button is-primary">
            <span class="icon">
              <Icon icon={faPlus} />
            </span>
            <span>New</span>
          </button>
        </div>
      </div>
    </div>
    <table class="table is-striped is-fullwidth is-hoverable">
      <thead>
        <tr>
          <th>Link</th>
          <th>Destination</th>
          <th>Description</th>
          <th>Public</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each links as link}
          <tr>
            <td>{link.name}</td>
            <td><a target="_blank" href={link.url}>{link.url}</a></td>
            <td>{link.description || ""}</td>
            <td>{link.public ? "Yes" : "No"}</td>
            <td>
              <button class="button">
                <span class="icon is-small">
                  <Icon icon={faEdit} />
                </span>
              </button>
              <button
                class="button"
                on:click={() => deleteLinkHandler(link.name)}
              >
                <span class="icon is-small">
                  <Icon icon={faTrashAlt} />
                </span>
              </button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</main>

<style>
  .container {
    margin-top: 30px;
  }

  td,
  th {
    vertical-align: middle;
  }
</style>

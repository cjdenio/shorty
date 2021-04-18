<script>
  import Modal from "./Modal.svelte";

  export let active;

  let name = "";
  let url = "";
  let root = false;
  let description = "";
  let publicLink = false;

  function resetForm() {
    name = "";
    url = "";
    root = false;
    description = "";
    publicLink = false;
  }

  $: {
    if (root) {
      name = "root";
    } else {
      name = "";
    }
  }
</script>

<Modal
  title="New Link"
  text="Create"
  style="primary"
  bind:active
  on:close={resetForm}
>
  <form>
    <div class="field">
      <label class="label" for="name">Link</label>
      <div class="field has-addons">
        <p class="control">
          <button class="button is-static">/</button>
        </p>
        <p class="control is-expanded">
          <input
            class="input"
            type="text"
            placeholder="e.g. gh"
            id="name"
            bind:value={name}
            disabled={root}
          />
        </p>
      </div>
    </div>

    <div class="field">
      <input
        id="root"
        type="checkbox"
        class="switch is-rounded"
        bind:checked={root}
      />
      <label for="root">Root URL</label>
      <span class="help" style="display: inline-block; margin-left: 10px"
        >Creates a redirect for the root URL</span
      >
    </div>

    <div class="field">
      <label for="url" class="label">URL</label>
      <p class="control">
        <input
          class="input"
          type="url"
          placeholder="e.g. https://github.com/cjdenio"
          id="url"
          bind:value={url}
        />
      </p>
    </div>

    <div class="field">
      <label class="label" for="description">Description</label>
      <div class="field">
        <p class="control is-expanded">
          <input
            class="input"
            type="text"
            id="description"
            bind:value={description}
          />
        </p>
      </div>
    </div>

    <div class="field">
      <input
        id="public"
        type="checkbox"
        class="switch is-rounded"
        bind:checked={publicLink}
      />
      <label for="public">Public</label>
      <!-- <p class="help">Displays this link on the public links page</p> -->
    </div>
  </form>
</Modal>

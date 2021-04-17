<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let active: boolean;
  export let title: string;
  export let style: "primary" | "danger";
  export let text: string;

  const dispatch = createEventDispatcher();

  function submit() {
    dispatch("submit");
  }
</script>

<div class="modal" class:is-active={active}>
  <div class="modal-background" />
  <div class="modal-card">
    <header class="modal-card-head">
      <p class="modal-card-title">{title}</p>
      <button
        class="delete"
        aria-label="close"
        on:click={() => (active = false)}
      />
    </header>
    <section class="modal-card-body">
      <slot />
    </section>
    <footer class="modal-card-foot">
      <button
        class={`button is-${style}`}
        on:click={() => {
          submit();
          active = false;
        }}>{text}</button
      >
      <button class="button" on:click={() => (active = false)}>Cancel</button>
    </footer>
  </div>
</div>

<style>
  .modal-card {
    transition: transform 100ms;
  }

  .modal-background:active + .modal-card {
    transform: scale(1.02);
  }
</style>

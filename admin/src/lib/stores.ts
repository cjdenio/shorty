import { writable } from "svelte/store";

const storedToken = localStorage.getItem("token");

export const token = writable<string | null>(storedToken);
token.subscribe((i) => {
  localStorage.setItem("token", i);
});

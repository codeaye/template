<script>
  import { appWindow } from "@tauri-apps/api/window";
  import { darkTheme } from "../store";
  import { fade } from "svelte/transition";
  import Icon from "./Icon.svelte";

  let theme;
  darkTheme.subscribe((value) => {
    theme = value;
  });

  function toggleTheme() {
    darkTheme.update((theme) => !theme);
  }
</script>

<body class="flex justify-end">
  <button
    class="w-fit h-fit hover:bg-zinc-800 hover:text-zinc-300 transition-container"
    on:click={toggleTheme}
  >
    {#if theme}
      <div transition:fade>
        <Icon name="sun" class="h-5 w-5 m-3.5" />
      </div>
    {:else}
      <div transition:fade>
        <Icon name="moon" class="h-5 w-5 m-3.5" />
      </div>
    {/if}</button
  >
  <button class="w-fit h-fit hover:bg-zinc-800 hover:text-zinc-300">
    <Icon name="cog" class="h-5 w-5 m-3.5" /></button
  >
  <button
    class="w-fit h-fit hover:bg-zinc-800 hover:text-zinc-300"
    on:click={appWindow.minimize}
  >
    <Icon name="minimize" class="h-4 w-4 m-4" /></button
  >
  <button
    class="w-fit h-fit hover:bg-zinc-800 hover:text-zinc-300"
    on:click={appWindow.toggleMaximize}
  >
    <Icon name="maximize" class="h-4 w-4 m-4" /></button
  >
  <button
    class="w-fit h-fit bg-red-700 hover:bg-red-800 hover:text-zinc-300"
    on:click={appWindow.close}
  >
    <Icon name="x" class="h-4 w-4 m-4" /></button
  >
</body>

<style>
  .transition-container {
    display: grid;
    grid-template-rows: 1fr;
    grid-template-columns: 1fr;
  }

  .transition-container > * {
    grid-row: 1;
    grid-column: 1;
  }
</style>

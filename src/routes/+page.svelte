<script lang="ts">
  import { onMount } from "svelte";
  import { fetchWallpapers } from "../lib/list";
  import { app } from "@tauri-apps/api";
  import DebugWindow from "../components/debug/DebugWindow.svelte";
  import DebugWindowItem from "../components/debug/DebugWindowItem.svelte";
  import {
    appVersion,
    cropMode,
    debugGeneral,
    input,
    loading,
    modifiersHeld,
    previousInput,
    tauriVersion,
    wallpapers,
  } from "$lib/state";
  import { get } from "svelte/store";
  import Footer from "../components/Footer.svelte";
  import Loading from "../components/Loading.svelte";
  import Empty from "../components/Empty.svelte";
  import WallpaperList from "../components/WallpaperList.svelte";
  import ScrollContainer from "../components/ScrollContainer.svelte";
  import InputHandler from "../components/InputHandler.svelte";

  let modifiers = ["Alt", "Control", "Meta", "Shift"];

  onMount(async () => {
    appVersion.set(await app.getVersion());
    tauriVersion.set(await app.getTauriVersion());
    wallpapers.set(await fetchWallpapers());
    loading.set(false);
  });

  function handleKeyDown(event: KeyboardEvent) {
    if (modifiers.includes(event.key)) {
      modifiersHeld.set([...get(modifiersHeld), event.key]);
    }
  }

  function handleKeyUp(event: KeyboardEvent) {
    if (modifiers.includes(event.key)) {
      modifiersHeld.set(
        get(modifiersHeld).filter((modifier) => modifier !== event.key),
      );
    }
  }
</script>

<InputHandler />

<svelte:window on:keydown={handleKeyDown} on:keyup={handleKeyUp} />

{#if $debugGeneral}
  <DebugWindow top={180}>
    <DebugWindowItem name="$modifiersHeld" value={$modifiersHeld} />
    <DebugWindowItem
      name="$modifiersHeld > 0"
      value={$modifiersHeld.length > 0}
    />
    <DebugWindowItem name="modifiers" value={modifiers} />
    <DebugWindowItem name="$loading" value={$loading} />
    <DebugWindowItem name="$wallpapers.length" value={$wallpapers.length} />
    <DebugWindowItem name="$cropMode" value={$cropMode} />
    <DebugWindowItem name="$previousInput" value={$previousInput} />
    <DebugWindowItem name="$input" value={$input} />
    <DebugWindowItem name="$appVersion" value={$appVersion} />
    <DebugWindowItem name="$tauriVersion" value={$tauriVersion} />
  </DebugWindow>
{/if}

<div
  id="app"
  class:cropmode={$cropMode}
  class:modifiersheld={$modifiersHeld.length > 0}
>
  <ScrollContainer>
    {#if $loading}
      <Loading />
    {:else if $wallpapers.length > 0}
      <WallpaperList />
    {:else}
      <Empty />
    {/if}
  </ScrollContainer>
  <Footer />
</div>

<style>
  :global(html, body, p, h1, h2, h3) {
    margin: 0;
    padding: 0;
  }

  #app {
    background-color: #222;
    color: #ddd;
    height: 100vh;
    width: 100vw;
    display: flex;
    flex-direction: column;
    font-family: monospace;
  }
</style>

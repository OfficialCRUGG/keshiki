<script lang="ts">
  import { onMount } from "svelte";
  import { ensureAppdir, fetchWallpapers } from "../lib/list";
  import Spinner from "../components/Spinner.svelte";
  import { path, app } from "@tauri-apps/api";
  import Wallpaper from "../components/Wallpaper.svelte";
  import { setWallpaper } from "$lib/wallpaper";
  import ScrollHandler from "../components/ScrollHandler.svelte";
  import DebugWindow from "../components/DebugWindow.svelte";
  import DebugWindowItem from "../components/DebugWindowItem.svelte";

  let loading = true;
  let wallpapers: string[] = [];
  let directoriesDir: string;
  let scrollContainer: HTMLDivElement | undefined;
  let appVersion: string | undefined = undefined;
  let tauriVersion: string | undefined = undefined;

  let modifiers = ["Alt", "Control", "Meta", "Shift"];
  let modifiersHeld: string[] = [];
  let cropMode = false;
  let input = "";
  let previousInput = "";

  let debugScroll = false;
  let debugGeneral = false;

  function toggleCropMode() {
    cropMode = !cropMode;
  }

  function randomWallpaper() {
    if (wallpapers.length === 0) return;
    const randomIndex = Math.floor(Math.random() * wallpapers.length);
    setWallpaper(wallpapers[randomIndex]);
  }

  async function loadWallpapers() {
    loading = true;
    wallpapers = await fetchWallpapers();
    loading = false;
  }

  onMount(async () => {
    appVersion = await app.getVersion();
    tauriVersion = await app.getTauriVersion();
    wallpapers = await fetchWallpapers();
    directoriesDir = await path.join(await ensureAppdir(), "directories");
    loading = false;
  });

  function handleKeyDown(event: KeyboardEvent) {
    if (modifiers.includes(event.key)) {
      modifiersHeld = [...modifiersHeld, event.key];
    }
  }

  function handleKeyUp(event: KeyboardEvent) {
    if (modifiers.includes(event.key)) {
      modifiersHeld = modifiersHeld.filter(
        (modifier) => modifier !== event.key
      );
    }
  }

  function handleKeyPress(event: KeyboardEvent) {
    const key = event.key.toLowerCase();
    switch (key) {
      case "c":
        toggleCropMode();
        break;
      case "r":
        randomWallpaper();
        break;
      case "i":
        loadWallpapers();
        break;
      case "backspace":
        input = input.slice(0, -1);
        break;
      case "escape":
        previousInput = input;
        input = "";
        break;
      case "enter":
        if (input.length > 0) {
          const index = Number(input);
          if (index > 0 && index <= wallpapers.length) {
            setWallpaper(wallpapers[index - 1]);
          }
          if (input === "x91") {
            debugScroll = !debugScroll;
          }
          if (input === "x92") {
            debugGeneral = !debugGeneral;
          }
          previousInput = input;
          input = "";
        }
        break;
      case "a":
      case "h":
        var inputNumber = Number(input);
        if (!inputNumber || isNaN(inputNumber) || inputNumber < 1) {
          input = String(wallpapers.length);
        } else {
          input =
            inputNumber === 1
              ? String(wallpapers.length)
              : String(inputNumber - 1);
        }
        break;
      case "d":
      case "l":
        var inputNumber = Number(input);
        if (
          !inputNumber ||
          isNaN(inputNumber) ||
          inputNumber > wallpapers.length
        ) {
          input = "1";
        } else {
          input =
            inputNumber >= wallpapers.length ? "1" : String(inputNumber + 1);
        }
        break;
      case "e":
        input = previousInput;
        break;
      case "0":
      case "1":
      case "2":
      case "3":
      case "4":
      case "5":
      case "6":
      case "7":
      case "8":
      case "9":
      case "x":
        input += key;
        break;
    }
  }
</script>

<svelte:window
  on:keydown={handleKeyDown}
  on:keyup={handleKeyUp}
  on:keypress={handleKeyPress}
/>

<ScrollHandler container={scrollContainer} {debugScroll} />

{#if debugGeneral}
  <DebugWindow top={180}>
    <DebugWindowItem name="modifiersHeld" value={modifiersHeld} />
    <DebugWindowItem
      name="modifiersHeld > 0"
      value={modifiersHeld.length > 0}
    />
    <DebugWindowItem name="modifiers" value={modifiers} />
    <DebugWindowItem name="loading" value={loading} />
    <DebugWindowItem name="wallpapers.length" value={wallpapers.length} />
    <DebugWindowItem name="cropMode" value={cropMode} />
    <DebugWindowItem name="previousInput" value={previousInput} />
    <DebugWindowItem name="input" value={input} />
    <DebugWindowItem name="appVersion" value={appVersion} />
    <DebugWindowItem name="tauriVersion" value={tauriVersion} />
  </DebugWindow>
{/if}

<div
  id="app"
  class:cropmode={cropMode}
  class:modifiersheld={modifiersHeld.length > 0}
>
  <div id="inner" bind:this={scrollContainer}>
    {#if loading}
      <div id="loading">
        <Spinner />
      </div>
    {:else if wallpapers.length > 0}
      <div id="wallpaper-list">
        {#each wallpapers as wallpaper, index}
          <Wallpaper
            {wallpaper}
            hidden={input.length > 0 && index + 1 !== Number(input)}
            index={index + 1}
            showIndex={modifiersHeld.length > 0}
          />
        {/each}
      </div>
    {:else}
      <div id="empty">
        <span>no wallpapers found</span>
        <span>add directories to {directoriesDir}</span>
      </div>
    {/if}
  </div>
  <div id="footer">
    <span>keshiki{appVersion ? ` v${appVersion}` : ""}</span>
    <span>
      {input}
    </span>
    <div class="buttonarray">
      <button on:click={toggleCropMode} class:active={cropMode}>
        {#if modifiersHeld.length > 0}
          <span class="underline">C</span>rop Mode
        {:else}
          C
        {/if}
      </button>
      <button on:click={randomWallpaper}>
        {#if modifiersHeld.length > 0}
          <span class="underline">R</span>andom Wallpaper
        {:else}
          R
        {/if}
      </button>
      <button on:click={loadWallpapers}>
        {#if modifiersHeld.length > 0}
          Rescan <span class="underline">I</span>mages
        {:else}
          I
        {/if}
      </button>
    </div>
  </div>
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

  #inner {
    height: 100%;
    overflow-y: auto;
    overflow-x: hidden;
  }

  #loading {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
  }

  #footer {
    border-top: 1px solid #444;
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 34px;
    padding: 0 10px;
  }

  #empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    text-align: center;
  }

  #wallpaper-list {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
    justify-items: center;
    gap: 1px;
  }

  #footer .buttonarray {
    display: flex;
  }

  #footer .buttonarray button {
    appearance: none;
    background: none;
    border: none;
    height: 34px;
    width: 34px;
    color: white;
    font-size: 16px;
    font-family: inherit;
    border-top: 2px solid transparent;
  }

  :global(.modifiersheld) #footer .buttonarray button {
    width: auto;
    padding: 0 10px;
  }

  :global(.modifiersheld) #footer .buttonarray button span.underline {
    text-decoration: underline;
  }

  #footer .buttonarray button:hover {
    background-color: #444;
  }

  #footer .buttonarray button.active {
    border-top: 2px solid white;
  }
</style>

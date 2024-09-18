<script lang="ts">
  import { onMount } from "svelte";
  import Spinner from "./Spinner.svelte";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { setWallpaper } from "$lib/wallpaper";

  export let wallpaper: string;
  export let hidden: boolean = false;
  export let index: number;
  export let showIndex: boolean = false;

  let loading = true;
  let assetUrl: string | undefined;

  onMount(async () => {
    assetUrl = convertFileSrc(wallpaper);
    loading = false;
  });

  function set() {
    if (loading || assetUrl === undefined) return;

    setWallpaper(wallpaper);
  }
</script>

<button class="wallpaper" class:hidden on:click={set}>
  {#if loading}
    <div class="wallpaper-loading">
      <Spinner />
    </div>
  {:else}
    <img src={assetUrl} alt="wallpaper" />
  {/if}
  {#if showIndex}
    <div class="wallpaper-index">{index}</div>
  {/if}
</button>

<style>
  .wallpaper {
    appearance: none;
    background: none;
    border: none;
    outline: none;
    padding: 0;
    margin: 0;
    width: 100%;
    aspect-ratio: 16 / 9;
    background-color: black;
    position: relative;
  }

  .wallpaper.hidden {
    opacity: 25%;
  }

  .wallpaper img {
    object-fit: contain;
    height: 100%;
    width: 100%;
  }

  :global(.cropmode) .wallpaper img {
    object-fit: cover;
    object-position: center;
  }

  .wallpaper-loading {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .wallpaper-index {
    position: absolute;
    top: 5px;
    right: 5px;
    background-color: rgba(0, 0, 0, 0.5);
    color: white;
    font-size: 15px;
    padding: 2px 6px;
  }
</style>

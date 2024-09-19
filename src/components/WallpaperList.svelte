<script lang="ts">
  import { input, modifiersHeld, wallpapers } from "$lib/state";

  import Wallpaper from "./Wallpaper.svelte";
</script>

<div id="wallpaper-list">
  {#each $wallpapers as wallpaper, index}
    <Wallpaper
      {wallpaper}
      hidden={$input.length > 0 && index + 1 !== Number($input)}
      index={index + 1}
      showIndex={$modifiersHeld.length > 0}
    />
  {/each}
</div>

<style>
  #wallpaper-list {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
    justify-items: center;
    gap: 1px;
  }
</style>

<script lang="ts">
  import { fetchWallpapers } from "$lib/list";
  import {
    appVersion,
    cropMode,
    input,
    loading,
    modifiersHeld,
    wallpapers,
  } from "$lib/state";
  import type { Button } from "$lib/types";
  import { setWallpaper } from "$lib/wallpaper";
  import { get } from "svelte/store";

  const buttons: Button[] = [
    {
      shortLabel: "C",
      longLabel: "_C_rop Mode",
      key: "c",
      action() {
        cropMode.set(!get(cropMode));
      },
      activeFn() {
        return $cropMode;
      },
    },
    {
      shortLabel: "R",
      longLabel: "_R_andom Wallpaper",
      key: "r",
      action() {
        if ($wallpapers.length === 0) return;
        const randomIndex = Math.floor(Math.random() * $wallpapers.length);
        setWallpaper($wallpapers[randomIndex]);
      },
      activeFn() {
        return false;
      },
    },
    {
      shortLabel: "I",
      longLabel: "_Rescan _I_mages",
      key: "i",
      async action() {
        loading.set(true);
        wallpapers.set(await fetchWallpapers());
        loading.set(false);
      },
      activeFn() {
        return false;
      },
    },
  ];

  buttons.forEach((button) => {
    button.active = button.activeFn();
  });

  function keyPress(event: KeyboardEvent) {
    const key = event.key.toLowerCase();
    buttons.forEach((button) => {
      if (button.key === key) {
        button.action();
        button.active = button.activeFn();
      }
    });
  }
</script>

<svelte:window on:keypress={keyPress} />

<div class="footer">
  <span>keshiki{$appVersion ? ` v${$appVersion}` : ""}</span>
  <span>
    {$input}
  </span>
  <div class="footer-buttons">
    {#each buttons as button}
      <button
        on:click={() => {
          button.action();
          button.active = button.activeFn();
        }}
        class:active={button.active}
      >
        {#if $modifiersHeld.length > 0}
          {@html button.longLabel.replace(
            /_(.)_/g,
            "<span class='underline'>$1</span>",
          )}
        {:else}
          {button.shortLabel}
        {/if}
      </button>
    {/each}
  </div>
</div>

<style>
  .footer {
    border-top: 1px solid #444;
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 34px;
    padding: 0 10px;
  }

  .footer .footer-buttons {
    display: flex;
  }

  .footer .footer-buttons button {
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

  .footer .footer-buttons button:hover {
    background-color: #444;
  }

  .footer .footer-buttons button.active {
    border-top: 2px solid white;
    color: white;
  }

  :global(.modifiersheld) .footer .footer-buttons button {
    width: auto;
    padding: 0 10px;
  }

  .footer .footer-buttons button :global(span.underline) {
    text-decoration: underline;
  }
</style>

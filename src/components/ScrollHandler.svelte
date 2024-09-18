<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import DebugWindow from "./DebugWindow.svelte";
  import DebugWindowItem from "./DebugWindowItem.svelte";

  export let debugScroll: boolean = false;
  export let container: HTMLDivElement | undefined;

  let scrollSpeed = 0;
  let acceleration = 0.5;
  let maxSpeed = 25;
  let deceleration = 0.5;
  let direction = 0;

  let isUpPressed = false;
  let isDownPressed = false;

  let animationFrameId: number | null = null;

  function handleKeyDown(event: KeyboardEvent) {
    const key = event.key.toLowerCase();
    switch (key) {
      case "w":
      case "k":
        isUpPressed = true;
        break;
      case "s":
      case "j":
        isDownPressed = true;
        break;
    }
  }

  function handleKeyUp(event: KeyboardEvent) {
    const key = event.key.toLowerCase();
    switch (key) {
      case "w":
      case "k":
        isUpPressed = false;
        break;
      case "s":
      case "j":
        isDownPressed = false;
        break;
    }
  }

  function scrollPage() {
    if (container === undefined) return;

    if (isUpPressed || isDownPressed) {
      scrollSpeed = Math.min(scrollSpeed + acceleration, maxSpeed);
    }

    if (isUpPressed) {
      direction = -1;
    } else if (isDownPressed) {
      direction = 1;
    } else {
      direction = 0;
      if (scrollSpeed > 0) {
        scrollSpeed = Math.max(scrollSpeed - deceleration, 0);
      }
    }

    if (scrollSpeed !== 0) {
      container.scrollBy(0, scrollSpeed * direction);
    }

    animationFrameId = requestAnimationFrame(scrollPage);
  }

  onMount(() => {
    animationFrameId = requestAnimationFrame(scrollPage);
  });

  onDestroy(() => {
    if (animationFrameId !== null) {
      cancelAnimationFrame(animationFrameId);
    }
  });
</script>

<svelte:window on:keydown={handleKeyDown} on:keyup={handleKeyUp} />

{#if debugScroll}
  <DebugWindow>
    <DebugWindowItem name="scrollSpeed" value={scrollSpeed} />
    <DebugWindowItem name="acceleration" value={acceleration} />
    <DebugWindowItem name="maxSpeed" value={maxSpeed} />
    <DebugWindowItem name="deceleration" value={deceleration} />
    <DebugWindowItem name="direction" value={direction} />
    <DebugWindowItem name="isUpPressed" value={isUpPressed} />
    <DebugWindowItem name="isDownPressed" value={isDownPressed} />
  </DebugWindow>
{/if}

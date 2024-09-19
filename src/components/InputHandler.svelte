<script lang="ts">
  import {
    debugGeneral,
    debugScroll,
    input,
    previousInput,
    wallpapers,
  } from "$lib/state";
  import { setWallpaper } from "$lib/wallpaper";
  import { get } from "svelte/store";

  function handleKeyPress(event: KeyboardEvent) {
    const key = event.key.toLowerCase();
    switch (key) {
      case "backspace":
        input.set(get(input).slice(0, -1));
        break;
      case "escape":
        previousInput.set(get(input));
        input.set("");
        break;
      case "enter":
        if (get(input).length > 0) {
          const index = Number(input);
          if (index > 0 && index <= $input.length) {
            setWallpaper($input[index - 1]);
          }

          if (get(input) === "x91") {
            debugScroll.set(!get(debugScroll));
          }
          if (get(input) === "x92") {
            debugGeneral.set(!get(debugGeneral));
          }

          previousInput.set(get(input));
          input.set("");
        }
        break;
      case "a":
      case "h":
        var inputNumber = Number($input);
        if (!inputNumber || isNaN(inputNumber) || inputNumber <= 1) {
          input.set(String($wallpapers.length));
        } else {
          input.set(String(inputNumber - 1));
        }
        break;
      case "d":
      case "l":
        var inputNumber = Number($input);
        console.log(inputNumber);
        if (
          !inputNumber ||
          isNaN(inputNumber) ||
          inputNumber + 1 > $wallpapers.length
        ) {
          console.log("a");
          input.set("1");
        } else {
          console.log("b");
          input.set(String(inputNumber + 1));
        }
        break;
      case "e":
        input.set(get(previousInput));
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
        input.set(get(input) + key);
        break;
    }
  }
</script>

<svelte:window on:keypress={handleKeyPress} />

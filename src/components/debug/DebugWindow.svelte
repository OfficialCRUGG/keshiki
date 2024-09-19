<script lang="ts">
  export let top: number = 0;
</script>

<div class="debug-window" style="top: {top}px">
  <table>
    <slot />
  </table>
</div>

<style>
  :global(.debug-window) {
    background-color: rgba(0, 0, 0, 0.75);
    color: white;
    font-size: 15px;
    position: fixed;
    top: 0;
    left: 0;
    z-index: 999;
    font-family: monospace;
    padding: 5px;
    min-width: 200px;
  }
  :global(.debug-window table tr td:first-child) {
    text-align: right;
    padding-right: 10px;
    color: rgb(255, 255, 255, 0.75);
  }
</style>

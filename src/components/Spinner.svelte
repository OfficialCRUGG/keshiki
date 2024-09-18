<svg class="container" viewBox="0 0 35 35" height="35" width="35">
  <rect
    class="track"
    x="2.5"
    y="2.5"
    fill="none"
    stroke-width="5px"
    width="32.5"
    height="32.5"
  />
  <rect
    class="car"
    x="2.5"
    y="2.5"
    fill="none"
    stroke-width="5px"
    width="32.5"
    height="32.5"
    pathlength="100"
  />
</svg>

<style>
  .container {
    --uib-size: 35px;
    --uib-color: white;
    --uib-speed: 1.2s;
    --uib-bg-opacity: 0.1;
    height: var(--uib-size);
    width: var(--uib-size);
    transform-origin: center;
    will-change: transform;
    overflow: visible;
  }

  .car {
    fill: none;
    stroke: var(--uib-color);
    stroke-dasharray: 25, 75;
    stroke-dashoffset: 0;
    animation: travel var(--uib-speed) linear infinite;
    will-change: stroke-dasharray, stroke-dashoffset;
    transition: stroke 0.5s ease;
  }

  .track {
    fill: none;
    stroke: var(--uib-color);
    opacity: var(--uib-bg-opacity);
    transition: stroke 0.5s ease;
  }

  @keyframes travel {
    0% {
      stroke-dashoffset: 0;
    }

    100% {
      stroke-dashoffset: -100;
    }
  }
</style>

<!-- src/routes/annotate/+page.svelte -->

<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { invoke } from '@tauri-apps/api/core';
  
    let imagePath = '';
    let projectId = '';
    let points: { x: string; y: string }[] = [];

  
    // Extract query parameters
    $: {
      const urlParams = new URLSearchParams($page.url.search);
      imagePath = urlParams.get('image') || '';
      projectId = urlParams.get('id') || '';
    }
  
    function handleClick(event: MouseEvent) {
      // Get click coordinates relative to the image
      const img = event.target as HTMLImageElement;
      const rect = img.getBoundingClientRect();
      const x = ((event.clientX - rect.left) / rect.width).toFixed(4);
      const y = ((event.clientY - rect.top) / rect.height).toFixed(4);
  
      if (!removePoint(parseFloat(x), parseFloat(y))) {
            points = [...points, { x, y }];
        }
    }

    function removePoint(x: number, y: number): boolean {
        const tolerance = 0.02; // 2% tolerance for clicking accuracy

        const index = points.findIndex(p =>
            Math.abs(parseFloat(p.x) - x) < tolerance && 
            Math.abs(parseFloat(p.y) - y) < tolerance
        );

        if (index !== -1) {
            points = [...points.slice(0, index), ...points.slice(index + 1)];
            return true;
        }
        return false;
    }

    function clearAnnotations() {
        points = []; // Reset points array
    }
  
    async function saveAnnotations() {
      try {
        await invoke('save_coordinates', {
          projectId,
          coordinates: points
        });
        alert('Coordinates saved!');
        goto(`/projectDetails?id=${projectId}`);
      } catch (error) {
        console.error('Error saving coordinates:', error);
      }
    }
  </script>
  
  <style>
    .container {
      text-align: center;
      padding: 20px;
      padding-bottom: 4rem;
    }
    .image-wrapper {
        position: relative;
        display: inline-block;
    }
    img {
      max-width: 100%;
      height: auto;
      cursor: crosshair;
    }
    .marker {
        position: absolute;
        width: 10px;
        height: 10px;
        background-color: red;
        border-radius: 50%;
        transform: translate(-50%, -50%);
        pointer-events: none; /* ✅ Allows clicks to pass through */
    }
    .coordinates {
      margin-top: 15px;
    }
    .button-group {
        margin-top: 20px;
        display: flex;
        gap: 10px;
        justify-content: center;
    }
  </style>
  
  <div class="container">
    <h1>Annotate Image</h1>
    
    {#if imagePath}
        <div class="image-wrapper">
            <img src={imagePath} alt="Annotate Image" on:click={handleClick} />
            
            <!-- Markers for clicked points -->
            {#each points as { x, y }}
                <div class="marker" style="left: {parseFloat(x) * 100}%; top: {parseFloat(y) * 100}%;"></div>
            {/each}
        </div>
    {/if}
  
    <div class="coordinates">
      <h3>Coordinates:</h3>
      <ul>
        {#each points as { x, y }, i}
          <li>Point {i + 1}: X: {x}, Y: {y}</li>
        {/each}
      </ul>
    </div>
  
    <div class="button-group">
        <button on:click={saveAnnotations}>Save Annotations</button>
        <button on:click={clearAnnotations}>Clear</button>
        <button on:click={() => goto(`/projectDetails?id=${projectId}`)}>Cancel</button>
    </div>
  </div>
  
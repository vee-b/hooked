<!-- src/routes/annotate/+page.svelte -->

<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { invoke } from '@tauri-apps/api/core';
    import { updateAnnotations, annotations } from '../../stores/projectsList';
  
    let imagePath = '';
    let projectId = '';
    let points: { lat: string; lng: string }[] = [];

  
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
      const lat = ((event.clientX - rect.left) / rect.width).toFixed(4);
      const lng = ((event.clientY - rect.top) / rect.height).toFixed(4);
  
      if (!removePoint(parseFloat(lat), parseFloat(lng))) {
            points = [...points, { lat, lng }];
        }
    }

    function removePoint(lat: number, lng: number): boolean {
        const tolerance = 0.02; // 2% tolerance for clicking accuracy

        const index = points.findIndex(p =>
            Math.abs(parseFloat(p.lat) - lat) < tolerance && 
            Math.abs(parseFloat(p.lng) - lng) < tolerance
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
  
//     async function saveAnnotations() {
//         annotations.update(currentAnnotations => {
//             return { ...currentAnnotations, [projectId]: points };
//         });
//         //alert('Annotations saved!');
//         goto(`/projectDetails?id=${projectId}`);
//     }

    // Function to update annotations in MongoDB
    async function saveAnnotations() {
        // Prepare data to be saved
        const annotationsData = points;

        try {
            // Make an API call or invoke Tauri command to update the MongoDB database
            await updateAnnotations(projectId, annotationsData);
            //alert('Annotations saved!');
            goto(`/projectDetails?id=${projectId}`);
        } catch (error) {
            console.error('Error saving annotations:', error);
            alert('Failed to save annotations.');
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
            {#each points as { lat, lng }}
                <div class="marker" style="left: {parseFloat(lat) * 100}%; top: {parseFloat(lng) * 100}%;"></div>
            {/each}
        </div>
    {/if}
  
    <div class="coordinates">
      <h3>Coordinates:</h3>
      <ul>
        {#each points as { lat, lng }, i}
          <li>Point {i + 1}: X: {lat}, Y: {lng}</li>
        {/each}
      </ul>
    </div>
  
    <div class="button-group">
        <button on:click={saveAnnotations}>Save Annotations</button>
        <button on:click={clearAnnotations}>Clear</button>
        <button on:click={() => goto(`/projectDetails?id=${projectId}`)}>Cancel</button>
    </div>
  </div>
  
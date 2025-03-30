<!-- src/routes/annotate/+page.svelte -->
 <!-- To be main branch page for adding coords/notes to coords -->

 <script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { invoke } from '@tauri-apps/api/core';
  import { Project } from '../../models/Project';
  import { updateAnnotations, annotations, fetchProjectById } from '../../stores/projectsList';
  import { checkLoginStatus } from '../../controllers/accountsController';
  
  let imagePath = '';
  let projectId = '';
  let project: Project | null = null;
  let points: { lat: string; lng: string; note: string[] }[] = [];
  let editNotesMode = false; // Toggle for enabling/disabling coordinates editing
  let currentNote = ''; // Stores the current note entered by the user

  // Fetch project details on mount
  onMount(async () => {
    // Check if user if logged in
    const isLoggedIn = checkLoginStatus();
    if (!isLoggedIn) {
      goto('/login'); // Redirect if not logged in
    }

    // Extract query parameters
    const urlParams = new URLSearchParams(window.location.search);
      imagePath = urlParams.get('image') || '';
      projectId = urlParams.get('id') || '';

      if (projectId) {
        try {
            // Fetch project details (including coordinates) from the database
            project = await fetchProjectById(projectId);

            if (project && project.coordinates) {
                points = project.coordinates.map(coord => ({
                    lat: coord.lat.toString(),
                    lng: coord.lng.toString(),
                    note: coord.note ?? []
                }));
            } else {
                console.error('No coordinates found for this project.');
                points = [];
            }
        } catch (error) {
            console.error('Error fetching project details:', error);
            points = [];
        }
    }
  });

  function toggleEditMode() {
    editNotesMode = !editNotesMode;
  }

  function handleClick(event: MouseEvent) {
    if (editNotesMode) return; // Prevent adding new points if in edit mode

    const img = event.target as HTMLImageElement;
    const rect = img.getBoundingClientRect();
    const clickLat = ((event.clientX - rect.left) / rect.width).toFixed(4);
    const clickLng = ((event.clientY - rect.top) / rect.height).toFixed(4);

    // Convert lat/lng to float for comparison
    const clickLatNum = parseFloat(clickLat);
    const clickLngNum = parseFloat(clickLng);

    // Define the tolerance (2% of the image width/height)
    const tolerance = 0.02;

    // Check if click is near an existing point
    let clickedPointIndex = points.findIndex(({ lat, lng }) => {
      const latNum = parseFloat(lat);
      const lngNum = parseFloat(lng);
      return (
        Math.abs(latNum - clickLatNum) <= tolerance &&
        Math.abs(lngNum - clickLngNum) <= tolerance
      );
    });

    if (clickedPointIndex !== -1) {
      // If user clicked near an existing marker, enable edit mode for that point
      editNotesMode = true;
      currentNote = points[clickedPointIndex].note.join(', '); // Load existing note
    } else {
      // Otherwise, add a new point
      points = [...points, { lat: clickLat, lng: clickLng, note: [] }];
      toggleEditMode(); // Enable edit mode for new point
    }
  }
  
  // Currently broken
  function cancelAnnotations() {
    if (!editNotesMode) return; // Only allow canceling if in edit mode

    if (points.length > 0) {
        const lastPoint = points[points.length - 1];

        // Check if note is empty
        if (lastPoint.note && lastPoint.note.length === 0) {
            points = points.slice(0, -1); // Remove last added marker
        }
    }
    currentNote = ''; // Clear any entered note
    toggleEditMode(); // Exit edit mode
  }

  async function clearAnnotations() {
      points = []; // Reset points array
      const annotationsData = points;
      await updateAnnotations(projectId, annotationsData);
      // Refresh the current page to reload the data
      await goto(`/annotations?id=${projectId}&image=${imagePath}`);
  }

  async function saveNote() {
    if (!currentNote.trim()) {
        alert('Please enter a note before saving.');
        return;
    }

    // Find the last clicked marker (or currently edited one)
    const selectedPointIndex = points.findIndex(point => point.note.join(', ') === currentNote);

    if (selectedPointIndex !== -1) {
        points[selectedPointIndex].note = [currentNote]; // Update the correct point's note
    } else {
        console.error('No matching point found. Adding new one.');
        // Optionally: Add the note to the last added point if no match is found
        if (points.length > 0) {
            points[points.length - 1].note = [currentNote];
        }
    }

    try {
        // Save the updated points array to the database
        await updateAnnotations(projectId, points);
        currentNote = ''; // Clear input
        toggleEditMode(); // Exit edit mode
        await goto(`/annotations?id=${projectId}&image=${imagePath}`);
    } catch (error) {
        console.error('Error saving annotations:', error);
        alert('Failed to save annotations.');
    }
  }

  function autoResize(event: Event) {
    const target = event.target as HTMLTextAreaElement; // Type assertion
    target.style.height = "auto"; // Reset height
    target.style.height = target.scrollHeight + "px"; // Adjust height
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
      /* background-color: black;
      background-color: greenyellow; */
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
  .note-input {
    margin-top: 10px;
    width: 90%;           /* Match image width */
    min-height: 50px;      /* Start small */
    max-width: 100%;
    font-size: 16px;       /* Bigger text */
    padding: 10px;
    border-radius: 5px;
    border: 1px solid #ccc;
    resize: none;          /* Prevent manual resizing */
    overflow: hidden;      /* Hide scrollbar */
    line-height: 1.5;      /* Better readability */
  }
</style>

<div class="container">
  <h1>Annotations</h1>
  
  {#if imagePath}
      <div class="image-wrapper">
          <img src={imagePath} alt="Annotate Image" on:click={handleClick} />
          
          
          {#each points as { lat, lng }}
              <div 
                class="marker" 
                style="left: {parseFloat(lat) * 100}%; top: {parseFloat(lng) * 100}%;"
                on:click={(event) => handleClick(event)}
                ></div>
          {/each}
      </div>
  {/if}

  <!-- <button on:click={toggleEditMode}>
      {editNotesMode ? 'Disable Note Editing' : 'Add Notes'}
    </button> -->

  <!-- <div class="coordinates">
    <h3>Coordinates:</h3>
    <ul>
      {#each points as { lat, lng}, i}
        <li>Point {i + 1}: X: {lat}, Y: {lng}</li>
      {/each}
    </ul>
  </div> -->

  {#if !editNotesMode}
    <button on:click={clearAnnotations}>Clear All Notes</button>
  {/if}

  {#if editNotesMode}
    <!-- <div class="note-input">
      <input type="text" bind:value={currentNote} placeholder="Enter your note" />
    </div> -->
    <textarea 
      bind:value={currentNote} 
      placeholder="Enter your note" 
      class="note-input"
      on:input={autoResize}
    ></textarea>
    <div class="button-group">
        <button on:click={saveNote}>Save Note</button>
        <!-- <button on:click={() => goto(`/annotations?id=${projectId}&image=${imagePath}`)}>Cancel</button> -->
        <button on:click={cancelAnnotations}>Cancel</button>
    </div>
  {/if}
</div>
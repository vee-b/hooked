<!-- src/routes/annotate/+page.svelte -->
 <!-- To be main branch page for adding coords/notes to coords -->

 <script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { invoke } from '@tauri-apps/api/core';
  import { Project } from '../../models/Project';
  import { updateAnnotations, annotations, fetchProjectById } from '../../stores/projectsList';
  
  let imagePath = '';
  let projectId = '';
  let project: Project | null = null;
  let points: { lat: string; lng: string; note: string[] }[] = [];
  let editNotesMode = false; // Toggle for enabling/disabling coordinates editing
  let currentNote = ''; // Stores the current note entered by the user

  // Fetch project details on mount
  onMount(async () => {
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
      //if (!editNotesMode) return; // Prevent changes if coordinates editing is disabled
      if (editNotesMode) return; // Only toggle on click if editMode is off

      // Get click coordinates relative to the image
      const img = event.target as HTMLImageElement;
      const rect = img.getBoundingClientRect();
      const lat = ((event.clientX - rect.left) / rect.width).toFixed(4);
      const lng = ((event.clientY - rect.top) / rect.height).toFixed(4);

      // Initialize the note as an empty array (or use dynamic input)
      const note: string[] = []; 

      points = [...points, { lat, lng, note }];
      toggleEditMode(); // Enable edit mode after the first click
  }

  // ...

  // function handleClick(event: MouseEvent) {
  //   if (editNotesMode) return; // Prevent adding new points if in edit mode

  //   const img = event.target as HTMLImageElement;
  //   const rect = img.getBoundingClientRect();
  //   const clickLat = ((event.clientX - rect.left) / rect.width).toFixed(4);
  //   const clickLng = ((event.clientY - rect.top) / rect.height).toFixed(4);

  //   // Convert lat/lng to float for comparison
  //   const clickLatNum = parseFloat(clickLat);
  //   const clickLngNum = parseFloat(clickLng);

  //   // Define the tolerance (2% of the image width/height)
  //   const tolerance = 0.02;

  //   // Check if click is near an existing point
  //   let clickedPointIndex = points.findIndex(({ lat, lng }) => {
  //       const latNum = parseFloat(lat);
  //       const lngNum = parseFloat(lng);
  //       return (
  //           Math.abs(latNum - clickLatNum) <= tolerance &&
  //           Math.abs(lngNum - clickLngNum) <= tolerance
  //       );
  //   });

  //   if (clickedPointIndex !== -1) {
  //       // If user clicked near an existing marker, enable edit mode for that point
  //       editNotesMode = true;
  //       currentNote = points[clickedPointIndex].note.join(', '); // Load existing note
  //   } else {
  //       // Otherwise, add a new point
  //       points = [...points, { lat: clickLat, lng: clickLng, note: [] }];
  //       toggleEditMode(); // Enable edit mode for new point
  //   }
  // }


  function cancelAnnotations() {
    if (points.length > 0) {
        points = points.slice(0, -1); // Remove only the last added marker
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

  // Function to update annotations in MongoDB
  async function saveNote() {
    // Ensure that the point has a note before saving
    if (points.length > 0 && currentNote.trim()) {
        // Add the note to the first point (since we are only dealing with one point at a time)
        points[0].note = [currentNote]; // Assign the note to the point

        // Clear the current note after saving
        const annotationsData = points;
        currentNote = ''; // Clear the input

        try {
            // Save the annotations in the database
            await updateAnnotations(projectId, annotationsData);
            // Toggle editMode to false
            toggleEditMode();
            // Refresh the current page to reload the data
            await goto(`/annotations?id=${projectId}&image=${imagePath}`);
        } catch (error) {
            console.error('Error saving annotations:', error);
            alert('Failed to save annotations.');
        }
    } else {
        alert('Please enter a note before saving.');
    }
  }

  // ...

  // async function saveNote() {
  //   if (!currentNote.trim()) {
  //       alert('Please enter a note before saving.');
  //       return;
  //   }

  //   const selectedPointIndex = points.findIndex(point => point.note.join(', ') === currentNote);

  //   if (selectedPointIndex !== -1) {
  //       points[selectedPointIndex].note = [currentNote]; // Update the existing note
  //   }

  //   try {
  //       await updateAnnotations(projectId, points);
  //       currentNote = ''; // Clear input
  //       toggleEditMode(); // Exit edit mode
  //       await goto(`/annotations?id=${projectId}&image=${imagePath}`);
  //   } catch (error) {
  //       console.error('Error saving annotations:', error);
  //       alert('Failed to save annotations.');
  //   }
  // }

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
  .note-input {
    margin-top: 10px;
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

  <div class="coordinates">
    <h3>Coordinates:</h3>
    <ul>
      {#each points as { lat, lng}, i}
        <li>Point {i + 1}: X: {lat}, Y: {lng}</li>
      {/each}
    </ul>
  </div>

  {#if !editNotesMode}
    <button on:click={clearAnnotations}>Clear All Notes</button>
  {/if}

  {#if editNotesMode}
    <div class="note-input">
      <input type="text" bind:value={currentNote} placeholder="Enter your note" />
      <!-- <button on:click={addNote}>Add Note</button> -->
    </div>
    <div class="button-group">
        <button on:click={saveNote}>Save Note</button>
        <!-- <button on:click={() => goto(`/annotations?id=${projectId}&image=${imagePath}`)}>Cancel</button> -->
        <button on:click={cancelAnnotations}>Cancel</button>
    </div>
  {/if}
</div>
<!-- src/routes/annotate/+page.svelte -->
 <!-- To be main branch page for adding coords/notes to coords -->

 <script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { invoke } from '@tauri-apps/api/core';
  import { Project } from '../../models/Project';
  import { updateAnnotations, annotations, fetchProjectById } from '../../stores/projectsList';
  import { checkLoginStatus, logoutAccount } from '../../controllers/accountsController';
  import { ArrowLeft } from 'lucide-svelte';
  
  let imagePath = '';
  let projectId = '';
  let project: Project | null = null;
  let points: { lat: string; lng: string; note: string[] }[] = [];
  let editNotesMode = false; // Toggle for enabling/disabling coordinates editing
  let currentNote = ''; // Stores the current note entered by the user
  let selectedPointIndex: number | null = null;
  let originalPosition: { lat: string; lng: string } | null = null;


  // Handle logout on button click
  const handleLogout = () => {
    logoutAccount();
  };

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
    const img = event.target as HTMLImageElement;
    const rect = img.getBoundingClientRect();
    const clickLat = ((event.clientX - rect.left) / rect.width).toFixed(4);
    const clickLng = ((event.clientY - rect.top) / rect.height).toFixed(4);

    const clickLatNum = parseFloat(clickLat);
    const clickLngNum = parseFloat(clickLng);

    const tolerance = 0.02;

    if (editNotesMode) {
      // Update position of the marker being edited
      if (selectedPointIndex !== null && points[selectedPointIndex]) {
        points[selectedPointIndex].lat = clickLat;
        points[selectedPointIndex].lng = clickLng;
        points = [...points]; // Trigger reactivity
      }
      return;
    }

    // Not in edit mode — check for existing point to edit
    let clickedPointIndex = points.findIndex(({ lat, lng }) => {
      const latNum = parseFloat(lat);
      const lngNum = parseFloat(lng);
      return (
        Math.abs(latNum - clickLatNum) <= tolerance &&
        Math.abs(lngNum - clickLngNum) <= tolerance
      );
    });

    if (clickedPointIndex !== -1) {
      // Enter edit mode for existing point
      selectedPointIndex = clickedPointIndex;

      originalPosition = {
      lat: points[clickedPointIndex].lat,
      lng: points[clickedPointIndex].lng,
    };

      editNotesMode = true;
      currentNote = points[clickedPointIndex].note.join(', ');
    } else {
      // Add new point
      const newPoint = { lat: clickLat, lng: clickLng, note: [] };
      points = [...points, newPoint];
      selectedPointIndex = points.length - 1;
      
      originalPosition = {
        lat: newPoint.lat,
        lng: newPoint.lng,
      };

      editNotesMode = true;
      currentNote = '';
    }
  }

  function cancelAnnotations() {
    if (!editNotesMode) return;

    if (selectedPointIndex !== null) {
      // Check if it's a newly added point with no note
      if (points[selectedPointIndex]?.note.length === 0) {
        points.splice(selectedPointIndex, 1);
      } else if (originalPosition) {
        // Revert to original position
        points[selectedPointIndex].lat = originalPosition.lat;
        points[selectedPointIndex].lng = originalPosition.lng;
      }
      points = [...points]; // Trigger reactivity
    }

    selectedPointIndex = null;
    originalPosition = null;
    currentNote = '';
    toggleEditMode();
  }


  async function clearAnnotations() {
      points = []; // Reset points array
      const annotationsData = points;
      await updateAnnotations(projectId, annotationsData);
      // Refresh the current page to reload the data
      await goto(`/annotations?id=${projectId}&image=${imagePath}`);
  }

  async function removeMarker() {
    if (!editNotesMode) return;

    // Attempt to find the marker being edited
    const markerIndex = points.findIndex(
      (point) => point.note.join(', ') === currentNote
    );

    if (markerIndex !== -1) {
      points.splice(markerIndex, 1); // Remove from array
      points = [...points]; // Trigger reactive update
    } else {
      // Fallback: Remove last marker if it has no note
      const lastPoint = points[points.length - 1];
      if (lastPoint && (!lastPoint.note || lastPoint.note.length === 0)) {
        points = points.slice(0, -1);
      }
    }

    currentNote = '';
    editNotesMode = false;

    // Save the updated points to the database
    try {
      await updateAnnotations(projectId, points);
    } catch (error) {
      console.error('Error saving annotation updates after removal:', error);
    }
  }


  async function saveNote() {
    if (!currentNote.trim()) {
        alert('Please enter a note before saving.');
        return;
    }

    // Find the last clicked marker (or currently edited one)
    let selectedPointIndex: number | null = points.findIndex(point => point.note.join(', ') === currentNote);

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

        selectedPointIndex = null;

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

  function navigateBack() {
    if (history.length > 1) {
      history.back(); // Navigate to previous page
    } else {
      goto('/projectDetails');
    }
  }
</script>

<style>
  .back-button-wrapper {
    position: absolute;
    top: 1rem;
    left: 1rem;
  }

  .back-button {
    display: flex;
    align-items: center;
    background: none;
    border: none;
    width: 45px;
    height: 45px;
    cursor: pointer;
    border-radius: 8px;
    transition: background 0.3s ease;
    margin-left: 1rem;
  }

  .logout-button-wrapper {
    position: absolute;
    top: 1rem;
    right: 1rem; /* Place the logout button at the top right */
  }

  .logout-button {
    display: flex;
    align-items: center;
    background: none;
    border: none;
    width: 80px;
    height: 45px;
    cursor: pointer;
    border-radius: 8px;
    transition: background 0.3s ease;
    margin-right: 1rem;
  }

  .title {
    color: rgb(57, 57, 57);
    font-size: 2rem;
    font-weight: lighter;
    margin-bottom: 20px;
    margin-top: 40px;
    text-align: start;
    letter-spacing: 8px; 
  }

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
  <h1 class="title">Annotations</h1>

  <div class="back-button-wrapper">
    <button class="back-button" on:click={navigateBack}>
      <ArrowLeft />
    </button>
  </div>

  <div class="logout-button-wrapper">
    <button class="logout-button" on:click={handleLogout}>
      Logout
    </button>
  </div>
  
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
        <button on:click={saveNote}>Save Note/s</button>
        <button on:click={removeMarker}>Remove Marker & Notes</button>
        <!-- <button on:click={() => goto(`/annotations?id=${projectId}&image=${imagePath}`)}>Cancel</button> -->
        <button on:click={cancelAnnotations}>Cancel</button>
    </div>
  {/if}
</div>
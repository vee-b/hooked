<!-- src/routes/annotate/+page.svelte -->
 <!-- To be main branch page for adding coords/notes to coords -->
  
<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { Project } from '../../models/Project';
  import { updateAnnotations, fetchProjectById } from '../../stores/projectsList';
  import { checkLoginStatus } from '../../controllers/accountsController';
  import { ArrowLeft } from 'lucide-svelte';
  import ConfirmationBox from '../../components/ConfirmationBox.svelte';

  let imagePath = '';
  let projectId = '';
  let project: Project | null = null;
  let points: { x: string; y: string; note: string[] }[] = [];
  let editNotesMode = false; // Toggle for enabling/disabling coordinates editing
  let currentNote = ''; // Stores the current note entered by the user
  let selectedPointIndex: number | null = null;
  let originalPosition: { x: string; y: string } | null = null;
  let selectedColor = 'white'; // Default marker color
  let showRemoveAllConfirm = false;
  let showDeleteConfirm = false;
  let showCancelConfirm = false;

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
        points = (project?.coordinates ?? []).map(coord => ({
          x: coord.lat.toString(),
          y: coord.lng.toString(),
          note: coord.note ?? []
        }));
      } catch (err) {
        console.error('Error loading project:', err);
      }
    }
  });

  function handleClick(event: MouseEvent) {
    const img = event.target as HTMLImageElement;
    const rect = img.getBoundingClientRect();
    const x = ((event.clientX - rect.left) / rect.width).toFixed(4);
    const y = ((event.clientY - rect.top) / rect.height).toFixed(4);

    const xNum = parseFloat(x);
    const yNum = parseFloat(y);
    const tolerance = 0.04;

    if (editNotesMode) {
      // Update position of the marker being edited
      if (selectedPointIndex !== null) {
        points[selectedPointIndex].x = x;
        points[selectedPointIndex].y = y;
        points = [...points]; // Trigger reactivity
      }
      return;
    }

    const index = points.findIndex(({ x, y }) =>
      Math.abs(parseFloat(x) - xNum) <= tolerance &&
      Math.abs(parseFloat(y) - yNum) <= tolerance
    );

    if (index !== -1) {
      // Enter edit mode for existing point
      selectedPointIndex = index;
      originalPosition = { x: points[index].x, y: points[index].y };
      editNotesMode = true;
      currentNote = points[index].note.join(', ');
    } else {
      // Add new point
      const newPoint = { x, y, note: [] };
      points = [...points, newPoint];
      selectedPointIndex = points.length - 1;
      originalPosition = { x, y };
      editNotesMode = true;
      currentNote = '';
    }
  }

  function toLatLngFormat() {
    return points.map(p => ({ lat: p.x, lng: p.y, note: p.note }));
  }

  async function saveNote() {
    if (!currentNote.trim()) {
      alert('Please enter a note before saving.');
      return;
    }

    if (selectedPointIndex !== null) {
      points[selectedPointIndex].note = [currentNote];
    } else if (points.length) {
      points[points.length - 1].note = [currentNote];
    }

    try {
      await updateAnnotations(projectId, toLatLngFormat());
      exitEdit();
    } catch (err) {
      console.error('Error saving notes:', err);
    }
  }

  async function removeMarker() {
    if (selectedPointIndex !== null) {
      points.splice(selectedPointIndex, 1); // Remove from array
      points = [...points]; // Trigger reactive update
      await updateAnnotations(projectId, toLatLngFormat());
      exitEdit();
    }
  }

  async function clearAnnotations() {
    points = []; // Reset points array
    await updateAnnotations(projectId, []);
    // Refresh the current page to reload the data
    await goto(`/annotations?id=${projectId}&image=${imagePath}`);
  }

  function cancelAnnotations() {
    if (selectedPointIndex !== null) {
      if (points[selectedPointIndex].note.length === 0) {
        // Remove the marker if no note was added
        points.splice(selectedPointIndex, 1);
      } else if (originalPosition) {
        // Otherwise, revert its position
        points[selectedPointIndex].x = originalPosition.x;
        points[selectedPointIndex].y = originalPosition.y;
      }
      points = [...points]; // Trigger reactivity
    }
    exitEdit();
  }

  function exitEdit() {
    selectedPointIndex = null;
    originalPosition = null;
    currentNote = '';
    editNotesMode = false;
  }

  function autoResize(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    target.style.height = 'auto';
    target.style.height = target.scrollHeight + 'px';
  }

  function navigateBack() {
    history.length > 1 ? history.back() : goto('/projectDetails');
  }
</script>

<style>
  .home { 
    text-align: center; 
    padding: 1rem; 
    color: black; }
  .header-container {
    display: flex; 
    justify-content: space-between; 
    align-items: center;
    padding: 1.5rem 2rem; 
    flex-wrap: wrap; 
    gap: 0;
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
    box-shadow: 5px 5px 10px rgba(0,0,0,0.1), -5px -5px 10px #fff;
    transition: box-shadow 0.2s ease;
  }
  .back-button:hover {
    box-shadow: inset 3px 3px 6px rgba(0,0,0,0.1), inset -3px -3px 6px #fff;
  }
  .title {
    color: rgb(57,57,57); 
    font-size: 2rem; 
    letter-spacing: 8px;
    text-align: start; 
    margin-bottom: 20px;
  }
  .divider { 
    height: 10px; 
    margin: 20px 0; 
    border-top: 1px solid #ccc; 
    }

  .button {
    display: flex; 
    align-items: center; 
    justify-content: center;
    gap: 0.5rem; /* spacing between icon and label */
    padding: 0.75rem 1rem; 
    border: none; 
    border-radius: 10px;
    font-size: 1rem; 
    cursor: pointer; 
    background: #fff;
    box-shadow: 5px 5px 10px rgba(0,0,0,0.1), -5px -5px 10px #fff;
    transition: box-shadow 0.2s ease;
    width: 100%;
  }

  .button:hover {
    box-shadow: inset 3px 3px 6px rgba(0,0,0,0.1), inset -3px -3px 6px #fff;
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
    width: 20px; 
    height: 20px; 
    background: white;
    border-radius: 50%; 
    transform: translate(-50%, -50%);
    display: flex; 
    justify-content: center; 
    align-items: center;
    font-size: 12px; 
    font-weight: bold; 
    color: black; 
    pointer-events: none; /* Allows clicks to pass through */
    user-select: none;
  }

  .button-group {
    margin: 20px 0; 
    display: flex; 
    gap: 10px; 
    justify-content: center;
  }

  .note-input {
    margin-top: 10px; 
    width: 90%; 
    min-height: 50px; 
    max-width: 100%;
    font-size: 16px; 
    padding: 10px; 
    border-radius: 5px;
    border: 1px solid #ccc; 
    resize: none; 
    overflow: hidden; 
    line-height: 1.5;
  }
</style>

<div class="home">
  <div class="header-container">
    <button class="back-button" on:click={navigateBack}><ArrowLeft /></button>
    <h1 class="title">Annotate</h1>
  </div>
  <div class="divider"></div>

  {#if imagePath}
    <div class="image-wrapper">
      <img src={imagePath} class="image-preview" alt="Annotate Image" on:click={handleClick} />
      {#each points as { x, y }, i}
        <div
          class="marker"
          style="left: {parseFloat(x) * 100}%; top: {parseFloat(y) * 100}%;
                 background: {selectedColor}; color: {selectedColor === 'black' ? 'white' : 'black'};"
        >
          {i + 1}
        </div>
      {/each}
    </div>
  {/if}

  {#if !editNotesMode}
    <button class="button" on:click={() => showRemoveAllConfirm = true}>
      Remove All Markers/Notes
    </button>
    <div class="button-group">
      Marker Colours:
      <button on:click={() => selectedColor = 'white'} style="background:white; border-radius:10px;"></button>
      <button on:click={() => selectedColor = 'black'} style="background:black; border-radius:10px;"></button>
    </div>
  {/if}

  {#if editNotesMode}
    <textarea
      bind:value={currentNote}
      placeholder="Enter your note"
      class="note-input"
      on:input={autoResize}
    ></textarea>
    <div class="button-group">
      <button class="button" on:click={saveNote}>Save</button>
      <button class="button" on:click={() => showDeleteConfirm = true}>Delete</button>
      <button class="button" on:click={() => showCancelConfirm = true}>Cancel</button>
    </div>
  {/if}
</div>

{#if showRemoveAllConfirm}
  <ConfirmationBox 
    message="Are you sure you want to remove all markers and notes?"
    onConfirm={async () => {
      showRemoveAllConfirm = false;
      await clearAnnotations();
    }}
    onCancel={() => showRemoveAllConfirm = false}
  />
{/if}

{#if showDeleteConfirm}
  <ConfirmationBox 
    message="Are you sure you want to remove this marker?"
    onConfirm={async () => {
      showDeleteConfirm = false;
      await removeMarker();
    }}
    onCancel={() => showDeleteConfirm = false}
  />
{/if}

{#if showCancelConfirm}
  <ConfirmationBox 
    message="Cancel editing this marker? Any unsaved note changes will be lost."
    onConfirm={() => {
      showCancelConfirm = false;
      cancelAnnotations();
    }}
    onCancel={() => showCancelConfirm = false}
  />
{/if}

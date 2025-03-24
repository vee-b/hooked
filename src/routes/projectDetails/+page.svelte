<!-- src/routes/projectDetails/+page.svelte -->

<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/core';
  import { Camera } from 'lucide-svelte';
  import { Camera as CapacitorCamera, CameraResultType } from '@capacitor/camera';
  import { Capacitor } from '@capacitor/core';
  import { addProject, editProject, fetchProjectById, sanitizeFileName, annotations, initializeProjectsList } from '../../stores/projectsList';
  import { Project } from '../../models/Project';

  // Form state and mode indicator
  let projectId: string | undefined = undefined;  // Changed to undefined instead of null
  let isEditMode = false;
  let project: Project | null = null;
  let projectCoordinates: { lat: number; lng: number }[] = [];
  let imagePath: string = 'No Image';
  let imagePreview: string = '/images/default-girl.jpg';
  let attempts: string = '0';
  let selectedOption: string = ''; // grade
  let dateTime: Date = new Date();
  let isSent: boolean = false;
  let isActive: boolean = true;
  let message: string = '';
  let imageFile: File | null = null;
  const options = ['V0', 'V1', 'V2', 'V3', 'V4', 'V5', 'V6', 'V7', 'V8', 'V9', 'V10'];

  // Helper function to format the Date to "YYYY-MM-DDThh:mm"
  function formatDateForInput(date: Date): string {
    const pad = (n: number) => n < 10 ? '0' + n : n;
    return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}T${pad(date.getHours())}:${pad(date.getMinutes())}`;
  }

  // A reactive variable that holds the formatted date string
  $: formattedDateTime = formatDateForInput(dateTime);

  // When the input changes, update the dateTime variable
  function updateDateTime(e: Event) {
    const target = e.target as HTMLInputElement;
    dateTime = new Date(target.value);
  }
  
  // Fetch query params for projectId
  $: {
    const urlParams = new URLSearchParams(window.location.search);
    projectId = urlParams.get('id') || '';
  }

  // On mount, check if we are editing an existing project.
  onMount(async () => {
    annotations.subscribe(store => {
      if (projectId && store[projectId]) {
        // Convert string coordinates to numbers
        projectCoordinates = store[projectId].map(coord => ({
          lat: parseFloat(coord.x),  // Convert x to a number
          lng: parseFloat(coord.y),  // Convert y to a number
        }));
        console.log('Project Coordinates:', projectCoordinates);
      } else {
        projectCoordinates = []; // Ensure empty array for "new_project"
        console.log('No coordinates for this project, initializing empty coordinates array.');
      }
    });

    const urlParams = new URLSearchParams(window.location.search);
    projectId = urlParams.get('id') ?? undefined; // Use undefined for new project
    if (projectId) {
      isEditMode = true;
      try {
        project = await fetchProjectById(projectId);
        if (project) {
          imagePath = project.image_path || 'No Image';
          imagePreview = project.image_path || '/images/default-girl.jpg';
          attempts = project.attempts ? project.attempts.toString() : '0';
          selectedOption = project.grade || '';
          isSent = project.is_sent || false;
          dateTime = project.date_time ? new Date(project.date_time) : new Date();
          isActive = project.is_active || true;
          projectCoordinates = project.coordinates || [];

          // Log project data to verify everything is fetched correctly
          console.log('Fetched Project:', project);
        }
      } catch (error) {
        console.error("Error fetching project:", error);
        message = "Error fetching project data";
      }
    }
  });

  // Function to capture/upload an image
  const pickImage = async () => {
    try {
      const image = await CapacitorCamera.getPhoto({
        quality: 90,
        allowEditing: true,
        resultType: CameraResultType.Uri,
      });
      const sanitizedFileName = sanitizeFileName(image);
      imagePreview = image.webPath || '/images/default-girl.jpg';
      imagePath = image.path || image.webPath || '';
      imageFile = await fetchImageFile(image, sanitizedFileName);
    } catch (error) {
      // @ts-ignore
      if (error.code !== 'CANCELLED') {
        console.error('Error capturing image:', error);
        message = 'Error capturing image.';
      }
    }
  };

  // Helper to convert the captured image into a File object.
  async function fetchImageFile(image: any, fileName: string): Promise<File | null> {
    try {
      const response = await fetch(image.webPath);
      const blob = await response.blob();
      const file = new File([blob], fileName);
      return file;
    } catch (error) {
      console.error('Error fetching image file:', error);
      return null;
    }
  }

  // Unified submit function
  async function submitData() {
    try {
      const dateTimeObj = new Date(dateTime);
      const attemptsNumber = parseInt(attempts, 10);

      // Log the coordinates before submitting
      console.log('Submitting Coordinates:', projectCoordinates);

      // Create a new Project instance; include _id only if editing.
      const currentProject = new Project({
        _id: projectId ? projectId : undefined,
        date_time: dateTimeObj,
        image_path: imagePath, // this may be replaced after image upload
        is_sent: isSent,
        attempts: attemptsNumber,
        grade: selectedOption,
        is_active: isActive,
        coordinates: projectCoordinates,
      });

      if (isEditMode) {
        // Update project: pass imageFile if a new image was captured.
        await editProject(currentProject, imageFile ?? undefined);
        message = 'Project updated successfully!';
      } else {
        // Add project: imageFile is required.
        if (imageFile) {
          await addProject(currentProject, imageFile);
          message = 'Project added successfully!';
        } else {
          console.error('No image file selected for new project.');
          message = 'Please select an image.';
          return;
        }
      }
      resetForm();
    } catch (error) {
      console.error('Error submitting project:', error);
      message = isEditMode ? 'Failed to update project.' : 'Failed to add project.';
    }
  }

  // Reset form fields after submission.
  function resetForm() {
    imagePath = 'No Image';
    imagePreview = '/images/default-girl.jpg';
    attempts = '0';
    selectedOption = '';
    dateTime = new Date();
    isSent = false;
    isActive = true;
    imageFile = null;
  }

  function navigateToHome() {
    goto('/');
  }
</script>

<style>
  .container {
    max-width: 400px;
    margin: 0 auto;
    font-family: Arial, sans-serif;
    padding: 2rem;
    padding-bottom: 4rem;
  }
  .image-wrapper {
    position: relative;
    display: inline-block;
    width: 100%;  /* Ensure it scales with the width of the container */
  }
  img {
    width: 100%;  /* Ensures image scales with its container */
    height: auto;
    display: block;
  }
  .marker {
    position: absolute;
    width: 15px;
    height: 15px;
    background-color: red;
    border-radius: 50%;
    transform: translate(-50%, -50%); /* Center the marker at the coordinates */
  }
  h1 { text-align: center; }
  .form-group { margin-bottom: 15px; }
  label { display: block; font-weight: bold; margin-bottom: 5px; }
  input[type='text'],
  input[type='number'],
  input[type='datetime-local'],
  select {
    width: 100%;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 4px;
    box-sizing: border-box;
  }
  input[type='checkbox'] { margin-right: 5px; }
  button {
    display: block;
    width: 100%;
    padding: 12px;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    font-size: 16px;
    margin-top: 10px;
    margin-bottom: 10px;
  }
  button:hover { background-color: #0056b3; }
  .button-row { display: flex; justify-content: space-between; gap: 10px; }
  .image-preview { width: 100%; height: auto; margin: 15px 0; display: block; }
  .message { color: green; text-align: center; margin-top: 10px; }
</style>

<div class="container">
  <h1>{isEditMode ? "Edit Project" : "Add Project"}</h1>

  {#if message}
    <p class="message">{message}</p>
  {/if}

  <div class="button-row">
    <button on:click={pickImage}>
      <Camera /> Capture/Upload Image
    </button>
  </div>

  {#if imagePreview}
    <img src={imagePreview} alt="Selected Image" class="image-preview" />
  {/if}

  {#if isEditMode}
  <button on:click={() => goto(`/annotations?id=${projectId}&image=${encodeURIComponent(imagePath)}&points=${JSON.stringify(projectCoordinates)}`)}>
    Annotations
  </button>
  {/if}

  <div>
    {#if isEditMode}
      {#if projectCoordinates.length > 0}
        <h3>Coordinates:</h3>
        <ul>
          {#each projectCoordinates as coord}
            <li>Latitude: {coord.lat}, Longitude: {coord.lng}</li>
          {/each}
        </ul>
      {:else}
        <p>No coordinates yet available for this project.</p>
      {/if}
    {/if}
    
  </div>

  <div class="form-group">
    <label for="dateTime">Date & Time</label>
    <input
      type="datetime-local"
      id="dateTime"
      bind:value={formattedDateTime}
      on:change={updateDateTime}
    />
  </div>

  <div class="form-group">
    <label for="grade">Grade</label>
    <select id="grade" bind:value={selectedOption}>
      <option value="" disabled selected>Select grade</option>
      {#each options as option}
        <option value={option}>{option}</option>
      {/each}
    </select>
  </div>

  <div class="form-group">
    <label for="attempts">Attempts</label>
    <input type="number" id="attempts" bind:value={attempts} />
  </div>

  <div class="form-group">
    <label for="isSent">Sent</label>
    <input type="checkbox" id="isSent" bind:checked={isSent} />
  </div>

  <div class="form-group">
    <label for="isActive">Active</label>
    <input type="checkbox" id="isActive" bind:checked={isActive} />
  </div>

  <div class="form-group">
    <button on:click={submitData}>
      {isEditMode ? "Update Project" : "Add Project"}
    </button>
  </div>

  <div class="button-row">
    <button on:click={navigateToHome}>Back to Home</button>
  </div>
</div>
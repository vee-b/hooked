<!-- src/routes/projectDetails/+page.svelte -->

<!-- 
When 'isEditMode' == true, the project update/edit page is shown.
When 'isEditMode' == false, the add new project page is shown.
-->

<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/core';
  import { Camera, ArrowLeft, LogOut } from 'lucide-svelte';
  import { Camera as CapacitorCamera, CameraResultType } from '@capacitor/camera';
  import { Capacitor } from '@capacitor/core';
  import { addProject, editProject, fetchProjectById, sanitizeFileName, annotations, initializeProjectsList } from '../../stores/projectsList';
  import { Project } from '../../models/Project';
  import { checkLoginStatus, logoutAccount } from '../../controllers/accountsController';
  import { gradeSystem, getCurrentGrades, convertVScaleGrade, convertFontScaleGrade } from '../../stores/settingsStore';
  import { fade } from 'svelte/transition';

  // Form state and mode indicator
  let projectId: string | undefined = undefined;  // Changed to undefined instead of null
  let isEditMode = false;
  let project: Project | null = null;
  let projectCoordinates: { lat: number; lng: number }[] = [];
  let imagePath: string = 'No Image';
  let imagePreview: string = '/images/logo.png';
  let attempts: string = '0';
  let selectedOption: string = ''; // grade
  let dateTime: Date = new Date();
  let inputDateTime = formatDateForInput(dateTime);

  let isSent: boolean = false;
  let isActive: boolean = true;
  let message: string = '';
  let imageFile: File | null = null;
  const options = ['V0', 'V1', 'V2', 'V3', 'V4', 'V5', 'V6', 'V7', 'V8', 'V9', 'V10', 'V11', 'V12', 'V13', 'V14', 'V15', 'V16', 'V17'];

  const allStyles = ['Trad', 'Top Rope', 'Bouldering', 'Topout', 'Traverse', 'Vert', 'Overhang', 'Slab', 'Roof', 'Static', 'Dyno', 'Technical', 'Reachy', 'Sustained', 'Power', 'Campusing', 'Slopers', 'Crimps', 'Jugs', 'Pinches', 'Pockets', 'Undercut', 'Side Pull', 'Hidden Hold/s', 'Volumes'];
  let selectedStyles: string[] = [];

  $: currentGrades = getCurrentGrades($gradeSystem);
  $: selectedOption = convertVScaleGrade(selectedOption, $gradeSystem);

  // Handle logout on button click
  // const handleLogout = () => {
  //   logoutAccount();
  // };

  function toggleStyle(style: string) {
    if (selectedStyles.includes(style)) {
      selectedStyles = selectedStyles.filter(s => s !== style);
    } else {
      selectedStyles = [...selectedStyles, style];
    }
    console.log('Selected styles being submitted:', selectedStyles);

  }

  function formatDateForInput(date: Date): string {
    const pad = (n: number) => n.toString().padStart(2, "0");

    const year = date.getFullYear();
    const month = pad(date.getMonth() + 1);
    const day = pad(date.getDate());
    const hours = pad(date.getHours());
    const minutes = pad(date.getMinutes());

    return `${year}-${month}-${day}T${hours}:${minutes}`;
  }

  function handleInputChange(e: Event) {
    const target = e.target as HTMLInputElement;
    inputDateTime = target.value;
    if (inputDateTime) {
      dateTime = new Date(inputDateTime);
    }
  }
  
  // Fetch query params for projectId
  $: {
    const urlParams = new URLSearchParams(window.location.search);
    projectId = urlParams.get('id') || '';
  }

  // On mount, check if we are editing an existing project.
  onMount(async () => {
    // Check if user if logged in
    const isLoggedIn = checkLoginStatus();
    if (!isLoggedIn) {
      goto('/login'); // Redirect if not logged in
    }
    
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
          imagePreview = project.image_path || '/images/logo.png';
          attempts = project.attempts ? project.attempts.toString() : '0';
          selectedOption = project.grade || '';
          //selectedOption = convertVScaleGrade(project.grade, $gradeSystem);
          isSent = project.is_sent || false;
          dateTime = project.date_time ? new Date(project.date_time) : new Date();
          inputDateTime = formatDateForInput(dateTime);
          isActive = project.is_active !== undefined ? project.is_active : true;
          projectCoordinates = project.coordinates || [];
          selectedStyles = project.style || [];

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
      imagePreview = image.webPath || '/images/logo.png';
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

      // Before creating the project, if the current grade system is "Font Scale",
      // convert the selected grade back to its v-scale equivalent.
      let gradeToStore = selectedOption;
      if ($gradeSystem === "Font Scale") {
        gradeToStore = convertFontScaleGrade(selectedOption, $gradeSystem);
      }

      // Log the coordinates before submitting
      console.log('Submitting Coordinates:', projectCoordinates);

      // Create a new Project instance; include _id only if editing.
      const currentProject = new Project({
        _id: projectId ? projectId : undefined,
        date_time: dateTimeObj,
        image_path: imagePath, // this may be replaced after image upload
        is_sent: isSent,
        attempts: attemptsNumber,
        grade: gradeToStore,
        is_active: isActive,
        coordinates: projectCoordinates,
        style: selectedStyles,
      });

      if (isEditMode) {
        // Update project: pass imageFile if a new image was captured.
        await editProject(currentProject, imageFile ?? undefined);
        message = 'Project updated successfully!';
        setTimeout(() => message = '', 3000);
      } else {
        if (!imageFile) {
          // Fetch and use the default logo image
          const response = await fetch('/images/logo.png');
          const blob = await response.blob();
          imageFile = new File([blob], 'logo.png', { type: blob.type });
          imagePath = '/images/logo.png';
        }
        await addProject(currentProject, imageFile);
        message = 'Project added successfully!';
        // goto(`/`);

        // if (history.length > 1) {
        //   history.back(); // Navigate to previous page
        // } else {
        //   goto('/');
        // }
      }
      await resetForm();
    } catch (error) {
      console.error('Error submitting project:', error);
      message = isEditMode ? 'Failed to update project.' : 'Failed to add project.';
    }
  }

  // Reset from with updated project details
  async function resetForm() {
    if (!projectId) {
      console.warn('No projectId provided for reset.');
      return;
    }

    try {
      const updatedProject = await fetchProjectById(projectId);

      if (!updatedProject) {
        console.warn('Updated project not found.');
        return;
      }

      project = updatedProject;
      imagePath = updatedProject.image_path || 'No Image';
      imagePreview = updatedProject.image_path || '/images/logo.png';
      attempts = updatedProject.attempts?.toString() || '0';
      selectedOption = updatedProject.grade || '';
      isSent = updatedProject.is_sent ?? false;
      dateTime = updatedProject.date_time ? new Date(updatedProject.date_time) : new Date();
      isActive = updatedProject.is_active ?? true;
      projectCoordinates = updatedProject.coordinates || [];
      selectedStyles = updatedProject.style || [];

      console.log('Form populated with updated project.');
    } catch (error) {
      console.error('Error fetching updated project:', error);
      message = 'Error fetching updated project data.';
    }
  }

  function navigateToHome() {
    if (history.length > 1) {
      history.back(); // Navigate to previous page
    } else {
      goto('/');
    }
  }
</script>

<style>
  .back-button-wrapper {
    /* display: inline; */
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

  /* .logout-button-wrapper {
    position: absolute;
    top: 1rem;
    right: 1rem; 
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
  }  */

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
    background-color: #ff8000b9;
    border-radius: 50%;
    transform: translate(-50%, -50%); /* Center the marker at the coordinates */
  }

  h1 { 
    text-align: center; 
  }

  .form-group { 
    margin-bottom: 15px; 
  }

  label { 
    display: inline-block; 
    font-weight: bold; 
    margin-bottom: 5px; 
  }

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

  input[type='checkbox'] { 
    margin-right: 5px; 
  }

  button {
    display: block;
    width: 100%;
    padding: 12px;
    padding: 1rem;
    border: none;
    border-radius: 10px;
    font-size: 1rem;
    cursor: pointer;
    background: #e6f4fd;
    box-shadow: 5px 5px 10px #b4d1e3, -5px -5px 10px #ffffff;
    transition: all 0.3s ease;
  }

  button:hover { 
    box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #e6f4fd;
  }

  .button-row { 
    display: flex; 
    justify-content: 
    space-between; 
    gap: 10px; 
  }

  .image-preview { 
    width: 100%; 
    height: auto; 
    margin: 15px 0; 
    display: block; 
  }

  .message { 
    color: green; 
    text-align: center; 
    margin-top: 10px; 
  }
</style>

<div class="container">  
  <h1 class="title">{isEditMode ? "Edit Project" : "Add Project"}</h1>

  <div class="back-button-wrapper">
    <button class="back-button" on:click={navigateToHome}>
      <ArrowLeft />
    </button>
  </div>

  <!-- <div class="logout-button-wrapper">
    <button class="logout-button" on:click={handleLogout}>
      Logout
    </button>
  </div> -->

  <div class="button-row">
    <button on:click={pickImage}>
      <Camera /> 
    </button>
  </div>

  {#if imagePreview}
    <img src={imagePreview} alt="Selected Image" class="image-preview" />
  {/if}

  {#if isEditMode}
    <button on:click={() => goto(`/annotations?id=${projectId}&image=${encodeURIComponent(imagePath)}`)}>
      Edit Annotations
    </button>
  {/if}

  <div>
    {#if isEditMode}
      {#if projectCoordinates.length > 0}
        <p>Notes: {projectCoordinates.length}</p>
      {:else}
        <p>Notes: 0</p>
      {/if}
    {/if}
    
  </div>

  <div class="form-group">
    <label for="dateTime">Date & Time</label>
    <input
      type="datetime-local"
      id="dateTime"
      bind:value={inputDateTime}
      on:change={handleInputChange}
    />
  </div>

  <div class="form-group">
    <label for="grade">Grade</label>
    <select id="grade" bind:value={selectedOption}>
      <option value="" disabled selected>Select grade</option>
      {#each currentGrades as grade}
        <option value={grade}>{grade}</option>
      {/each}
    </select>
  </div>

  <!-- CHECKBOXES -->
  <div class="styles-container">
    {#each allStyles as style}
      <label class="checkbox-style">
        <input
          type="checkbox"
          checked={selectedStyles.includes(style)}
          on:change={() => toggleStyle(style)}
        />
        {style}
      </label>
    {/each}
  </div>


  <div class="form-group">
    <label for="attempts">Attempts</label>
    <input type="number" id="attempts" bind:value={attempts} />
  </div>

  <div class="form-group">
    <label for="isSent" class="checkbox-item">Sent</label>
    <input type="checkbox" id="isSent" bind:checked={isSent} />
  </div>

  <div class="form-group">
    <label for="isActive" class="checkbox-item">Active</label>
    <input type="checkbox" id="isActive" bind:checked={isActive} />
  </div>

  <div class="form-group">
    <button on:click={submitData}>
      {isEditMode ? "Update Project" : "Add Project"}
    </button>
  </div>

  {#if message}
    <p class="message" transition:fade>{typeof message === 'string' ? message : JSON.stringify(message)}</p>
  {/if}
</div>
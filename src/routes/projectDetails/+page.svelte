<script>
  import { invoke } from '@tauri-apps/api/core';
  import { addProject } from '../../stores/projectsList';
  import { Project } from '../../models/Project';
  import { goto } from '$app/navigation';
  import { Camera, Upload } from 'lucide-svelte';
  import { Camera as CapacitorCamera, CameraResultType, CameraSource } from '@capacitor/camera'; // Import Capacitor Camera
  import { Filesystem, Directory } from '@capacitor/filesystem';
  import { Capacitor } from '@capacitor/core';

  let imagePath = 'No Image';
  let imagePreview = '/images/default-girl.jpg'; // Default image
  let attempts = '0';
  let grade = 'Unknown';
  let isSent = false;
  let selectedOption = 'Unknown';
  let dateTime = new Date();
  let isActive = true;
  let message = '';
  const options = ['V0', 'V1', 'V2', 'V3', 'V4', 'V5', 'V6', 'V7', 'V8', 'V9', 'V10'];

  const navigateToHome = () => {
    goto('/'); // Navigate to home page
  };

  async function submitData() {
    const dateTimeObj = new Date(dateTime);
    const attemptsNumber = parseInt(attempts, 10);

    const project = new Project({
      date_time: dateTimeObj,
      image_path: imagePath || 'No Image',
      is_sent: isSent,
      attempts: attemptsNumber,
      grade: selectedOption,
      is_active: isActive, 
    });

    try {
      await addProject(project);
      message = 'Project added successfully!';
    } catch (error) {
      console.error('Error adding project:', error);
      message = 'Failed to add project.';
    }
  }

  // Function to open the camera and capture an image
  const pickImage = async () => {
    try {
      const image = await CapacitorCamera.getPhoto({
        quality: 90,
        allowEditing: true,
        resultType: CameraResultType.Uri, // Use URI to display image
      });
      
      // Set the image preview to the captured image
      imagePreview = image.webPath || '/images/default-girl.jpg';; // Set the image path for preview
      imagePath = image.path || image.webPath || ''; // Store the image path in imagePath
    } catch (error) {
      // @ts-ignore
      if (error.code !== 'CANCELLED') { // Check if the action was cancelled
        console.error('Error capturing image:', error);
        message = 'Error capturing image.';
      } else {
        console.log('Camera action was cancelled.');
      }
    }
  };

</script>

<style>
  .container {
    max-width: 400px;
    margin: 0 auto;
    font-family: Arial, sans-serif;
    padding: 2rem;
    padding-bottom: 4rem; /* Space for the bottom navbar */
  }

  h1 {
    text-align: center;
  }

  .form-group {
    margin-bottom: 15px;
  }

  label {
    display: block;
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
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    font-size: 16px;
    margin-top: 10px;
  }

  button:hover {
    background-color: #0056b3;
  }

  .button-row {
    display: flex;
    justify-content: space-between;
    gap: 10px; /* Space between buttons */
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
  <h1>Project Details</h1>

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

  <div class="form-group">
    <label for="dateTime">Date & Time</label>
    <input type="datetime-local" id="dateTime" bind:value={dateTime} />
  </div>

  <!-- <div class="form-group">
    <label for="imagePath">Image Path</label>
    <input type="text" id="imagePath" bind:value={imagePath} placeholder="Image Path" />
  </div> -->

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
    <input type="number" id="attempts" bind:value={attempts} placeholder="Attempts" />
  </div>

  <div class="form-group">
    <label for="isSent">Sent</label>
    <input type="checkbox" id="isSent" bind:checked={isSent} />
  </div>

  <button on:click={submitData}>Add Project</button>
</div>
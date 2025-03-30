<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { deleteProject, projectsList } from "../stores/projectsList";
    import type { Project } from '../models/Project';
    import { goto } from '$app/navigation'; // Import goto for navigation
    import { gradeSystem, convertVScaleGrade } from '../stores/settingsStore'; // Import grade system store

    // /**
    //  * @type {{ id: any; formatted_date_time: any; image_path: any; grade: any; is_sent: any; attempts: any; is_active: any; }}
    //  */
    // @ts-ignore

  //   /**
  //  * @type {{ id: any; formatted_date_time: string; image_path: any; grade: any; is_sent: any; attempts: any; }}
  //  */
  export let project: Project; // Receive project object as prop
  const shortened_time = project.formatted_date_time.slice(10)
  const shortened_date = project.formatted_date_time.slice(0, 10)
  
  const dispatch = createEventDispatcher();

  // Reactive: get current system value
  $: currentSystem = $gradeSystem;
  $: displayedGrade = convertVScaleGrade(project.grade, currentSystem);

  // Navigation to edit the project
  async function editProject() {
    // Navigate to the edit project page
    goto(`/projectDetails?id=${project._id}`);
  }
    
  // Function to delete the project
  async function handleDeleteProject() {
    console.log(`Project ID to delete:`, project?._id); // Debugging
    if (!project?._id) {
      console.error("Error: Project ID is undefined or invalid.");
      return;
    }
        
    try {
      // Call delete function from the projectsList store
      await deleteProject(project._id);

      // Update the store manually by filtering out the deleted project
      projectsList.update((projects) => {
        return projects.filter((p) => p._id !== project._id);
      });

      // Dispatch an event to notify parent about deletion
      dispatch('projectDeleted', project._id);

      // Force UI refresh by navigating to the same page
      await goto(window.location.pathname);

      } catch (error) {
        console.error('Error deleting project:', error);
      }
    }
  </script>
  
  <!-- <style>
    .project-card {
      background-color: #f9f9f9;
      padding: 16px;
      margin: 12px 0;
      border-radius: 8px;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
      padding-bottom: 2rem;
    }

    .project-label {
      font-weight: bold;
      color: #333;
    }

    .text-content {
      font-size: 16px;
      margin: 4px 0;
    }
  
    .button {
      margin-top: 8px;
      padding: 8px 12px;
      background-color: #007bff;
      color: white;
      border: none;
      border-radius: 4px;
      cursor: pointer;
    }

    .button:hover {
      background-color: #0056b3;
    }

    img {
      max-width: 200px;
      max-height: 200px;
      border-radius: 4px;
      margin-top: 8px;
    }
    
  </style> -->

  <style>
    body {
      background-color: #121212; /* Dark background */
      font-family: 'Poppins', sans-serif;
      margin: 0;
      padding: 0;
      color: #f5f5f5; /* Light text */
    }

    .project-card {
      background-color: #1e1e1e;
      padding: 20px;
      margin: 20px auto;
      border-radius: 10px;
      box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
      text-align: center;
      max-width: 500px;
    }

    .project-label {
      font-weight: bold;
      color: #f5f5f5;
      font-size: 1.2rem;
      margin-bottom: 8px;
    }

    .text-content {
      font-size: 1rem;
      margin-bottom: 10px;
      color: #d3d3d3;
    }

    .button {
      width: 100%;
      padding: 12px;
      margin-top: 10px;
      border-radius: 8px;
      font-size: 1rem;
      font-weight: bold;
      /* text-transform: uppercase; */
      cursor: pointer;
      transition: all 0.3s ease-in-out;
    }

    .button:hover {
      transform: scale(1.05);
    }

    .button:first-of-type {
      background-color: transparent;
      border: 2px solid #00ff8092;
      color: white;
    }

    .button:last-of-type {
      background-color: transparent;
      border: 2px solid #ff8000b9;
      color: white;
    }

    img {
      max-width: 100%;
      max-height: 300px;
      border-radius: 8px;
      margin-top: 15px;
    }
  </style>

  <div class="project-card">
      <p class="project-label">{shortened_date}</p>
      <p class="project-label">{shortened_time}</p>

    {#if project.image_path}
      <img src={project.image_path} alt="Project Image" />
      <p class="loading" style="display: none;">Loading Image from Cloudinary...</p>
    {/if}

    <!-- <p class="project-label">Grade: {project.grade}</p> -->
    <p class="project-label">
      Grade: { displayedGrade }
      <!-- {isValidGrade(project.grade) ? project.grade : 'Unknown Grade'} -->
    </p>

    <p class="project-label">Sent: {project.is_sent ? "Yes" : "No"}</p>

    <p class="project-label">Attempts: {project.attempts}</p>

    {#if project.coordinates && project.coordinates.length > 0}
      <p class="project-label">Notes: {project.coordinates.length}</p>
    {:else}
      <p class="project-label">Notes: 0</p>
    {/if}
  
    <button class="button" on:click={editProject}>
      Edit Project
    </button>
  
    <button class="button" on:click={handleDeleteProject}>
      Delete Project
    </button>
  </div>
<script lang="ts">
    import { createEventDispatcher, tick } from 'svelte';
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
      await tick(); // wait for DOM to mount
      await goto(window.location.pathname);

      } catch (error) {
        console.error('Error deleting project:', error);
      }
    }
  </script>

  <style>
  .card {
    background: #ffffff;
    border-radius: 1rem;
    margin: 1rem;
    overflow: hidden;
    /* box-shadow: 0 4px 12px rgba(0, 0, 0, 0.06); */
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.123), -5px -5px 10px #ffffff;
    font-family: 'Inter', sans-serif;
    display: flex;
    flex-direction: column;
  }

  .card img {
    width: 100%;
    height: auto;
    object-fit: cover;
  }

  .card-content {
    padding: 1rem;
  }

  .title-row {
    font-size: 1.125rem;
    font-weight: 600;
    color: #111827;
    margin-bottom: 0.5rem;
  }

  .info {
    font-size: 0.95rem;
    color: #374151;
    margin: 0.25rem 0;
  }

  .button-row {
    display: flex;
    flex-direction: row;
    gap: 0.75rem;
    margin-top: 1rem;
  }

  .btn {
      width: 100%;
      padding: 12px;
      border-radius: 8px;
      font-size: 1rem;
      font-weight: lighter;
      cursor: pointer;
      border: none;
      /* border-radius: 9999px;  */
      border-radius: 1rem;
      background: #ffffff;
      box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.123), -5px -5px 10px #ffffff;
      transition: all 0.3s ease;
    }

    .btn:hover {
      box-shadow: inset 3px 3px 6px rgba(0, 0, 0, 0.123), inset -3px -3px 6px #ffffff;
    }
</style>

<div class="card">
  {#if project.image_path}
    <img src={project.image_path} alt="Project" loading="lazy" />
  {/if}

  <div class="card-content">
    <div class="title-row">{shortened_date} • {shortened_time}</div>
    <div class="info">Grade: {displayedGrade}</div>
    <div class="info">Sent: {project.is_sent ? 'Yes' : 'No'} • Attempts: {project.attempts}</div>
    <div class="info">Notes: {project.coordinates?.length || 0}</div>

    <div class="button-row">
      <button class="btn btn-edit" on:click={editProject}>Edit</button>
      <button class="btn btn-delete" on:click={handleDeleteProject}>Delete</button>
    </div>
  </div>
</div>







<!-- ........................................................ -->

<!-- <style>
    body {
      background-color: #e6f4fd; /* Dark background */
      font-family: 'Poppins', sans-serif;
      margin: 0;
      padding: 0;
      color: black; /* Light text */
    }

    .project-card {
      background-color: #e6f4fd;
      padding: 20px;
      margin: 20px auto;
      border-radius: 10px;
      box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
      text-align: left;
      max-width: 500px;
    }

    .project-label {
      font-weight: lighter;
      color: black;
      font-size: 1rem;
      margin-bottom: 10px;
      text-align: start;
    }

    .text-content {
      font-size: 1rem;
      margin-bottom: 10px;
      color: #d3d3d3;
    }

    .button {
      width: 100%;
      padding: 12px;
      border-radius: 8px;
      font-size: 1rem;
      font-weight: lighter;
      cursor: pointer;
      border: none;
      background: #e6f4fd;
      box-shadow: 5px 5px 10px #b4d1e3, -5px -5px 10px #ffffff;
      transition: all 0.3s ease;
    }

    .button:hover {
      box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #ffffff;
    }

    img {
      max-width: 100%;
      max-height: 200px;
      border-radius: 8px;
      margin-top: 15px;
      /* box-shadow: 5px 5px 10px #b4d1e3, -5px -5px 10px #ffffff; */
      box-shadow: 10px 10px 20px #b4d1e3, -10px -10px 20px #ffffff;
    }
  </style>

  <div class="project-card">
      <p class="project-label">{shortened_date}</p>
      <p class="project-label">{shortened_time}</p>

    {#if project.image_path}
      <img src={project.image_path} alt="Project Image" />
      <p class="loading" style="display: none;">Loading Image from Cloudinary...</p>
    {/if}

    <p class="project-label">
      Grade: { displayedGrade }
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
  </div> -->
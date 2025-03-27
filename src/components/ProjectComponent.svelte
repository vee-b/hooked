<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { deleteProject, projectsList } from "../stores/projectsList";
    import type { Project } from '../models/Project';
    import { goto, invalidate } from '$app/navigation'; // Import goto for navigation

    // /**
    //  * @type {{ id: any; formatted_date_time: any; image_path: any; grade: any; is_sent: any; attempts: any; is_active: any; }}
    //  */
    // @ts-ignore

  //   /**
  //  * @type {{ id: any; formatted_date_time: string; image_path: any; grade: any; is_sent: any; attempts: any; }}
  //  */
  export let project: Project; // Receive project object as prop
    
    // Navigation to edit the project
    async function editProject() {
      // Navigate to the edit project page
      goto(`/projectDetails?id=${project._id}`);
    }

    const dispatch = createEventDispatcher();
    
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
    
    // // Helper function to format text like in the Flutter method
    // /**
    //  * @param {string} text
    //  */
    // function buildText(text) {
    //   return text;
    // }
    
  </script>
  
  <style>
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
    
  </style>

  <div class="project-card">
      <p class="project-label">Date & Time: {project.formatted_date_time}</p>

    {#if project.image_path}
      <img src={project.image_path} alt="Project Image" />
      <p class="loading" style="display: none;">Loading Image from Cloudinary...</p>
    {/if}

    <p class="project-label">Grade: {project.grade}</p>

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
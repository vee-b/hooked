<script lang="ts">
    import { createEventDispatcher, tick } from 'svelte';
    import { deleteProject, projectsList } from "../stores/projectsList";
    import type { Project } from '../models/Project';
    import { goto } from '$app/navigation'; // Import goto for navigation
    import { gradeSystem, convertVScaleGrade } from '../stores/settingsStore'; // Import grade system store
    import ConfirmationBox from './ConfirmationBox.svelte';

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
  let showConfirmationBox = false;

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
    <!-- <div class="info">Sent: {project.is_sent ? 'Yes' : 'No'} • Attempts: {project.attempts}</div> -->
    <div class="info">
      Sent: {project.is_sent ? 'Yes' : 'No'}
      {#if project.formatted_sent_date !== 'Not Sent'}
        • {project.formatted_sent_date}
      {/if}
      • Attempts: {project.attempts}
    </div>
    <div class="info">Notes: {project.coordinates?.length || 0}</div>

    <div class="button-row">
      <button class="btn btn-edit" on:click={editProject}>Edit</button>
      <button class="btn btn-delete" on:click={() => showConfirmationBox = true}>Delete</button>

    </div>
  </div>
</div>

{#if showConfirmationBox}
  <ConfirmationBox 
    message={`Are you sure you want to delete this project?`}
    onConfirm={async () => {
      showConfirmationBox = false;
      await handleDeleteProject();
    }}
    onCancel={() => showConfirmationBox = false}
  />
{/if}
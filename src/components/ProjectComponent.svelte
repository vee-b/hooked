<!-- src/components/ProjectComponent.svelte -->

<script lang="ts">
  import { createEventDispatcher, tick } from 'svelte';
  import { deleteProject, projectsList } from "../stores/projectsList";
  import type { Project } from '../models/Project';
  import { goto } from '$app/navigation'; // Import goto for navigation
  import { gradeSystem, convertVScaleGrade } from '../stores/settingsStore'; // Import grade system store
  import ConfirmationBox from './ConfirmationBox.svelte';

  // Receive the entire project object from the parent component
  export let project: Project; 

  // Format the datetime string to just show date & time separately
  const shortened_time = project.formatted_date_time.slice(10)
  const shortened_date = project.formatted_date_time.slice(0, 10)

  // Create a Svelte event dispatcher
  const dispatch = createEventDispatcher();

  // State for showing the confirmation box
  let showConfirmationBox = false;

  // Reactive: get current grading system (V-Scale / Font Scale) from global store
  $: currentSystem = $gradeSystem;
  // Compute the displayed grade label based on selected grading system
  $: displayedGrade = convertVScaleGrade(project.grade, currentSystem);

  // Navigate to the edit project page, passing the project ID in URL
  async function editProject() {
    goto(`/projectDetails?id=${project._id}`);
  }
    
  // Function to delete this project
  async function handleDeleteProject() {
    console.log(`Project ID to delete:`, project?._id); // Debugging
    if (!project?._id) {
      console.error("Error: Project ID is undefined or invalid.");
      return;
    }
        
    try {
      // Delete on backend (via store action)
      await deleteProject(project._id);

      // Remove it manually from our local store so UI updates immediately
      projectsList.update((projects) => {
        return projects.filter((p) => p._id !== project._id);
    });

    // Notify parent component via dispatched event
    dispatch('projectDeleted', project._id);

    // Wait for DOM updates then refresh the current page to force re-query
    await tick();
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
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.123), -5px -5px 10px #ffffff;
    font-family: 'Inter', sans-serif;
    display: flex;
    flex-direction: column;

    max-width: 600px; /* cap the width on large screens */
    width: 90%;       /* on small screens, still responsive */
    margin: 1rem auto; /* center horizontally */
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
  <!-- Show the project image if it exists -->
  {#if project.image_path}
    <img src={project.image_path} alt="Project" loading="lazy" />
  {/if}

  <div class="card-content">
    <!-- Date + time -->
    <div class="title-row">{shortened_date} • {shortened_time}</div>

    <!-- Grade converted into selected system -->
    <div class="info">Grade: {displayedGrade}</div>
    
    <!-- Sent status, optionally show sent date -->
    <div class="info">
      Sent: {project.is_sent ? 'Yes' : 'No'}
      {#if project.formatted_sent_date !== 'Not Sent'}
        • {project.formatted_sent_date}
      {/if}
      • Attempts: {project.attempts}
    </div>

    <!-- Number of annotation markers on this image -->
    <div class="info">Notes: {project.coordinates?.length || 0}</div>

    <!-- Edit & Delete buttons -->
    <div class="button-row">
      <button class="btn btn-edit" on:click={editProject}>Edit</button>
      <button class="btn btn-delete" on:click={() => showConfirmationBox = true}>Delete</button>

    </div>
  </div>
</div>

<!-- If the user clicks delete, show a confirmation modal -->
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
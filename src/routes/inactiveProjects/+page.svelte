<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import ProjectComponent from '../../components/ProjectComponent.svelte';
    import { writable } from 'svelte/store';
    import { PlusCircle, Filter } from 'lucide-svelte';
    import type { Project } from '../../models/Project';
    import { fetchInactiveProjects, deleteProject } from '../../stores/projectsList';
  
     export const projectsList = writable<Project[]>([]);
  
    const navigateToNewProject = () => {
      goto('/projectDetails');
    };
  
    const fetchProjects = async () => {
      try {
        const projectsData = await fetchInactiveProjects();
        projectsList.set(projectsData);  // projectsData should be Project[]
        console.log('Fetched projects successfully:', projectsData);
      } catch (error) {
        console.error('Error fetching active projects:', error);
      }
    };
  
    onMount(() => {
      fetchProjects();
    });
  
    let filterActive = false;
  
    const toggleFilter = () => {
      filterActive = !filterActive;
    };

    // Refresh project list after deletion
    async function handleDeleteProject(projectId: string) {
      try {
        // Call delete function from the store
        await deleteProject(projectId);

        // Fetch the updated project list after deletion
        fetchProjects(); // Manually re-fetch projects
      } catch (error) {
        console.error('Error deleting project:', error);
      }
    }
  </script>
  
  <style>
    .home {
      text-align: center;
      padding: 2rem;
      font-family: Arial, sans-serif;
    }
  
    .button {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      margin: 1rem;
      padding: 1rem;
      background-color: #007bff;
      color: white;
      border: none;
      border-radius: 5px;
      cursor: pointer;
      font-size: 16px;
    }
  
    .button:hover {
      background-color: #0056b3;
    }
  
    .button-container {
      display: flex;
      justify-content: space-between;
      margin-top: 1rem;
    }
  
    .divider {
      height: 30px;
      border-top: 1px solid #ccc;
      margin: 20px 0;
    }
  
    .title {
      font-size: 2rem;
      font-weight: bold;
      margin-bottom: 1rem;
    }
  </style>
  
  <div class="home">
    <h1 class="title">Inactive Projects</h1>
  
    <div class="button-container">
      <button class="button" on:click={toggleFilter}>
        <Filter /> {filterActive ? 'Hide Filter' : 'Show Filter'}
      </button>
    </div>
  
    <div class="divider"></div>
  
    {#each $projectsList as project (project._id)}
      <ProjectComponent {project} />
    {/each}
  </div>
  
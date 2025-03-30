<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import ProjectComponent from '../components/ProjectComponent.svelte';
  import { writable } from 'svelte/store';
  import { PlusCircle, Filter } from 'lucide-svelte';
  import type { Project } from '../models/Project';
  import { fetchActiveFilteredProjects } from '../stores/projectsList';
  import { checkLoginStatus } from '../controllers/accountsController';
  import { tick } from 'svelte';

  export const projectsList = writable<Project[]>([]);

  const navigateToNewProject = () => {
    goto('/projectDetails');
  };

  const fetchProjects = async (filters: { grade: string, isSent: boolean }) => {
    try {
      const projectsData = await fetchActiveFilteredProjects(filters.grade, String(filters.isSent));
      projectsList.set(projectsData);  // projectsData should be Project[]
      console.log('Fetched projects successfully:', projectsData);
    } catch (error) {
      console.error('Error fetching active projects:', error);
    }
  };

  onMount(() => {
    const isLoggedIn = checkLoginStatus();  // Check login status when the component mounts
    if (isLoggedIn) {
      const initialFilters = {
        grade: selectedGrade,
        isSent: isSent,
      };
      fetchProjects(initialFilters);  // Fetch projects initially with default filters
    } else {
      goto('/login'); // Redirect if not logged in
    }
  });

  let filterActive = false;
  let selectedGrade: string = '';
  let isSent: boolean;

  const toggleFilter = () => {
    filterActive = !filterActive;
  };

  const applyFilters = async () => {
    await tick(); // Wait for UI updates
    console.log('Applying Filters:', { selectedGrade, isSent });
    
    const filters = {
      grade: selectedGrade,
      isSent: isSent ?? false, // Defaults to false if null/undefined
    };

    console.log('Filters Object:', filters);
    fetchProjects(filters);
  };
</script>

<!-- <style>
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

  .filters {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 1rem;
  }

  .filter-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  select {
    padding: 8px;
    font-size: 14px;
    width: 200px;
  }
</style> -->

<style global>
  /* Body and Background */
  body {
      background-color: #121212; /* Dark background */
      font-family: 'Poppins', sans-serif;
      margin: 0;
      padding: 0;
      color: #f5f5f5; /* Light text for contrast */
      margin-bottom: 3rem;
    }

  .home {
    text-align: center;
    padding: 2rem;
    font-family: 'Poppins', sans-serif;
    background-color: #121212;
    color: #f5f5f5;
  }

  .project-components-container {
    margin-top: 1rem;
  }

  .button {
    padding: 1rem;
    border: none;
    border-radius: 10px;
    cursor: pointer;
    font-size: 1rem;
    width: 100%;
    transition: background-color 0.3s ease, transform 0.3s ease;
    border: 2px solid #00ff8092; 
    background-color: transparent;
    color: white;
  }

  .button:hover {
    background-color: #00ff8000;
    transform: scale(1.05);
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
    color: white;
    margin-bottom: 20px;
    -webkit-text-stroke: 1px #00000048;
  }

  .filters {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 1rem;
  }

  .filter-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  select {
    padding: 8px;
    font-size: 14px;
    width: 200px;
    background-color: #333;
    color: #f5f5f5;
    border: 2px solid #444;
    border-radius: 10px;
    transition: border-color 0.3s ease, box-shadow 0.3s ease;
  }

  select:focus {
    border-color: #00ff8092;
    box-shadow: 0 0 5px rgba(0, 123, 255, 0.5);
    outline: none;
  }

  input[type="checkbox"] {
    width: 20px;
    height: 20px;
  }

  .error-message {
    color: red;
    font-size: 0.875rem;
    margin-bottom: 10px;
  }
</style>

<div class="home">
  <h1 class="title">Active Projects</h1>

  <div class="button-container">
    <button class="button" on:click={toggleFilter}>
      <Filter /> {filterActive ? 'Hide Filter' : 'Show Filter'}
    </button>

    <button class="button" on:click={navigateToNewProject}>
      <PlusCircle /> Add New Project
    </button>
  </div>

  {#if filterActive}
    <div class="filters">
      <div class="filter-item">
        <label for="grade">Grade</label>
        <select id="grade" bind:value={selectedGrade}>
          <option value="">All Grades</option>
          {#each ['V0', 'V1', 'V2', 'V3', 'V4', 'V5', 'V6', 'V7', 'V8', 'V9', 'V10', 'V11', 'V12', 'V13', 'V14', 'V15', 'V16', 'V17'] as grade}
            <option value={grade}>{grade}</option>
          {/each}
        </select>
      </div>

      <div class="filter-item">
        <label for="isSent">Sent</label>
        <input type="checkbox" id="isSent" bind:checked={isSent} />
      </div>

      <button class="button" on:click={applyFilters}>Apply Filters</button>
    </div>
  {/if}

  <div class="divider"></div>

  <div class="project-components-container">
    {#each $projectsList as project (project._id)}
      <ProjectComponent {project} on:projectDeleted={() => fetchProjects({ grade: selectedGrade, isSent })} />
    {/each}
  </div>
</div>
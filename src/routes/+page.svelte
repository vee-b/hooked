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

<style global>
  /* Body and Background */
  body {
      background-color: #e6f4fd; /* Dark background */
      font-family: 'Poppins', sans-serif;
      margin: 0;
      padding: 0;
      color: black; /* Light text for contrast */
      margin-bottom: 3rem;
    }

  .home {
    text-align: center;
    padding: 1rem;
    font-family: 'Poppins', sans-serif;
    background-color: #e6f4fd;
    color: black;
  }

  .project-components-container {
    margin-top: 1rem;
  }

  .button {
    padding: 1rem;
    margin-bottom: 20px;
    border: none;
    width: 100%;
    border-radius: 10px;
    font-size: 1rem;
    cursor: pointer;
    background: #e6f4fd;
    box-shadow: 5px 5px 10px #b4d1e3, -5px -5px 10px #ffffff;
    transition: all 0.3s ease;
  }

  .button:hover {
    box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #ffffff;
  }

  .button-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    margin-top: 1rem;
  }

  .divider {
    height: 10px;
    border-top: 1px solid #ccc;
    margin: 20px 0;
    border-style: hidden;
    color: white;
  }

  .title {
    color: rgb(57, 57, 57);
    font-size: 2rem;
    font-weight: lighter;
    margin-bottom: 20px;
    text-align: start;
    letter-spacing: 8px; /* Adjust the value to control the space between letters */
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
    background-color: #e6f4fd !important;
    box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #ffffff !important;
    color: black !important;
    border: 2px solid #e6f4fd;
    border-radius: 10px;
    transition: border-color 0.3s ease, box-shadow 0.3s ease;
  }

  select:focus {
    border-color: #e6f4fd;
    box-shadow: inset 5px 5px 10px #b4d1e3, inset -5px -5px 10px #ffffff !important;
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

  <div class="divider"></div>

  <div class="button-container">
    <button class="button" on:click={toggleFilter}>
      <Filter /> {filterActive ? 'Hide Filter' : 'Show Filter'}
    </button>

    <button class="button" on:click={navigateToNewProject}>
      <PlusCircle /> New Project
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
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

  {#each $projectsList as project (project._id)}
    <ProjectComponent {project} on:projectDeleted={() => fetchProjects({ grade: selectedGrade, isSent })} />
  {/each}
</div>
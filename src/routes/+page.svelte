<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import ProjectComponent from '../components/ProjectComponent.svelte';
  import { writable } from 'svelte/store';
  import { PlusCircle, Filter } from 'lucide-svelte';
  import type { Project } from '../models/Project';
  import { fetchActiveProjects, fetchActiveFilteredProjects } from '../stores/projectsList';
  import { checkLoginStatus } from '../controllers/accountsController';
  import { tick } from 'svelte';
  import { slide } from 'svelte/transition'

  export const projectsList = writable<Project[]>([]);

  const navigateToNewProject = () => {
    goto('/projectDetails');
  };

  const fetchProjects = async () => {
  try {
    const projectsData = await fetchActiveProjects();
    projectsList.set(projectsData);
    console.log('Fetched active projects:', projectsData);
  } catch (error) {
    console.error('Error fetching active projects:', error);
  }
};

  const fetchFilteredProjects = async (filters: { grades: string[], isSent: boolean }) => {
    try {
      const projectsData = await fetchActiveFilteredProjects(filters.grades, String(filters.isSent));
      projectsList.set(projectsData);  // projectsData should be Project[]
      console.log('Fetched projects successfully:', projectsData);
    } catch (error) {
      console.error('Error fetching active projects:', error);
    }
  };

  onMount(() => {
    const isLoggedIn = checkLoginStatus();  // Check login status when the component mounts
    if (isLoggedIn) {
      fetchProjects();
    } else {
      goto('/login'); // Redirect if not logged in
    }
  });

  let filterActive = false;
  let selectedGrades: string[] = []; // Store multiple selected grades
  let isSent: boolean;

  const toggleFilter = () => {
    filterActive = !filterActive;
  };

  const applyFilters = async () => {
    await tick(); // Wait for UI updates
    console.log('Selected Grades:', selectedGrades);  // Log selected grades
    console.log('Applying Filters:', { selectedGrades, isSent });
    
    const filters = {
      grades: selectedGrades,
      isSent: isSent ?? false, // Defaults to false if null/undefined
    };

    console.log('Filters Object:', filters);
    fetchFilteredProjects(filters);
  };

  const clearFilters = async () => {
    selectedGrades = [];
    isSent = false;
    await tick(); // Ensure UI updates before fetching
    fetchProjects(); // Fetch unfiltered active projects
  }
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

  .filter-button,
  .button {
    padding: 1rem;
    margin-bottom: 5px;
    border: none;
    width: 100%;
    border-radius: 10px;
    font-size: 1rem;
    cursor: pointer;
    background: #e6f4fd;
    box-shadow: 5px 5px 10px #b4d1e3, -5px -5px 10px #ffffff;
    transition: all 0.3s ease;
  }

  .filter-button:hover,
  .button:hover {
    box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #e6f4fd;
  }

  /* .filter-button:hover {
    box-shadow: inset 3px 3px 6px #e6f4fd, inset -3px -3px 6px #e6f4fd;
  } */

  .button-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    margin-top: 1rem;
    margin-bottom: 1rem;
    /* box-shadow: 5px 5px 10px #b4d1e3, -5px -5px 10px #ffffff; */
    box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #e6f4fd;
    transition: all 0.3s ease;
    padding: 10px;
    border-radius: 10px;
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
    /* gap: 1rem; */
    /* margin-top: 1rem; */
  }

  .filter-item {
    display: flex;
    align-items: center;
    /* gap: 0.5rem; */
  }

  .filter-button-div {
    display: flex;
    flex-direction: row;
    gap: 10px; /* Adds space between items */
    justify-content: space-between; /* Distributes items evenly */
    align-items: center; /* Aligns items vertically */
    width: 100%; /* Ensures it takes full width */
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

  select[multiple] {
    height: 150px; /* Adjust height to show multiple options */
  }

  .checkbox-container {
    display: flex;
    flex-wrap: wrap; /* Wraps checkboxes to the next line when space is filled */
    gap: 10px; /* Spacing between checkboxes */
    justify-content: flex-start; /* Align checkboxes to the left */
    max-width: 100%; /* Ensures that the container does not overflow */
  }

  .checkbox-item {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  input[type="checkbox"] {
    width: 20px;
    height: 20px;
  }

  /* Style the scrollbar */
  .checkbox-container::-webkit-scrollbar {
    width: 8px;  /* Adjust the width of the scrollbar */
  }

  /* Style the thumb (the draggable part of the scrollbar) */
  .checkbox-container::-webkit-scrollbar-thumb {
    background-color: #b4d1e3;  /* Set a background color for the thumb */
    border-radius: 4px;  /* Round the edges of the thumb */
    border: 2px solid #e6f4fd;  /* Optional: Add a border around the thumb */
  }

  /* Style the track (the area the thumb moves within) */
  .checkbox-container::-webkit-scrollbar-track {
    background-color: #e6f4fd;  /* Set a background color for the track */
    border-radius: 4px;  /* Round the edges of the track */
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
    <button class="filter-button {filterActive ? 'active' : ''}" on:click={toggleFilter}>
      <Filter /> 
    </button>

    {#if filterActive}
      <div class="filters" transition:slide={{ duration: 300 }}> 

        <div class="filter-item">
          <label for="isSent">Sent</label>
          <input type="checkbox" id="isSent" bind:checked={isSent} />
        </div>

        <div class="filter-item">
          <!-- <label>Grade</label> -->
          <div class="checkbox-container" style="max-height: 150px; overflow-y: auto;">
            {#each ['V0', 'V1', 'V2', 'V3', 'V4', 'V5', 'V6', 'V7', 'V8', 'V9', 'V10', 'V11', 'V12', 'V13', 'V14', 'V15', 'V16', 'V17'] as grade}
              <div class="checkbox-item">
                <input 
                  type="checkbox" 
                  id={grade} 
                  value={grade} 
                  bind:group={selectedGrades} 
                />
                <label for={grade}>{grade}</label>
              </div>
            {/each}
          </div>
        </div>
        
        <div class="filter-button-div">
          <button class="button" on:click={applyFilters}>Apply Filters</button> 
          <button class="button" on:click={clearFilters}>Clear Filters</button>
        </div>
      </div>
    {/if}
  </div>

  <button class="button" on:click={navigateToNewProject}>
    <PlusCircle /> New Project
  </button>

  <div class="divider"></div>

  <div class="project-components-container">
    {#each $projectsList as project (project._id)}
      <ProjectComponent {project} on:projectDeleted={() => fetchActiveFilteredProjects()} />
      <!-- <ProjectComponent {project} /> -->
    {/each}
  </div>
</div>
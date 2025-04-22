<!-- src/routes/+page.svelte -->

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
  import { afterNavigate } from '$app/navigation';

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

  const fetchFilteredProjects = async (filters: { grades: string[], isSent?: boolean }) => {
    try {
      const isSentParam = filters.isSent !== undefined ? String(filters.isSent) : null;
      const projectsData = await fetchActiveFilteredProjects(filters.grades, String(filters.isSent));
      projectsList.set(projectsData);  // projectsData should be Project[]
      console.log('Fetched projects successfully:', projectsData);
    } catch (error) {
      console.error('Error fetching active projects:', error);
    }
  };

  onMount(async () => {
    const isLoggedIn = checkLoginStatus();  // Check login status when the component mounts
    if (isLoggedIn) {
      fetchProjects();
    } else {
      goto('/login'); // Redirect if not logged in
    }
  });

  afterNavigate(() => {
    fetchProjects(); // or fetchFilteredProjects if filters should persist
  });

  let filterActive = false;
  let selectedGrades: string[] = []; // Store multiple selected grades
  //let isSent: boolean;
  let isSent: boolean | null = null; // null = no filter applied
  let sentFilterValue: string = 'all';


  const toggleFilter = () => {
    filterActive = !filterActive;
  };

  const applyFilters = async () => {
    await tick(); // Wait for UI updates
    console.log('Selected Grades:', selectedGrades);  // Log selected grades
    console.log('Applying Filters:', { selectedGrades, isSent });
    
    const filters = {
      grades: selectedGrades,
      //isSent: isSent ?? false, // Defaults to false if null/undefined
      isSent: isSent !== null ? isSent : undefined // omit if null
    };

    console.log('Filters Object:', filters);
    fetchFilteredProjects(filters);
  };

  const clearFilters = async () => {
    selectedGrades = [];
    //isSent = false;
    isSent = null;
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

  .button {
    padding: 1rem;
    /* margin-bottom: 5px; */
    border: none;
    /* width: 100%; */
    max-width: 70px;
    border-radius: 10px;
    font-size: 1rem;
    cursor: pointer;
    background: #e6f4fd;
    box-shadow: 5px 5px 10px #b4d1e3, -5px -5px 10px #ffffff;
    transition: all 0.3s ease;
  }

  .button:hover {
    box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #e6f4fd;
  }

  .top-buttons-container {
    display: flex;
    flex-direction: row;
    gap: 10px;
  }

  .filter-button-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
    box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #e6f4fd;
    transition: all 0.3s ease;
    border-radius: 10px;
    max-width: 80%;
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
    /* max-width: fit-content; */
    max-width: 100%;
  }

  .sent-filter-container {
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  /* Make the sent and grade labels smaller */
  .sent-filter-container label,
  .filters > label {
    font-size: 0.9rem;
  }


  .filter-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    /* flex-direction: column; */
    /* justify-content: space-between; */
    /* margin-top: 1rem;
    margin-bottom: 1rem; */
    transition: all 0.3s ease;
    padding-left: 20px;
    padding-right: 20px;
    /* padding-bottom: 10px; */
    border-radius: 0px;
  }

  .filter-button-div {
    display: flex;
    flex-direction: row;
    gap: 10px; /* Adds space between items */
    justify-content: space-between; /* Distributes items evenly */
    align-items: center; /* Aligns items vertically */
    max-width: 100%; /* Ensures it takes full width */
    padding: 1rem;
    flex-wrap: wrap; /* in case buttons get too tight */
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
    gap: 10px; /* Spacing between checkboxes */
    justify-content: flex-start; /* Align checkboxes to the left */
    padding: 10px;
    max-width: 100%; /* Ensures that the container does not overflow */
    max-height: 250px; 
    font-size: 0.75rem; /* smaller font size */
    flex-wrap: wrap; /* Wraps checkboxes to the next line when space is filled */
    overflow-y: auto;

    /* flex-wrap: nowrap;
    overflow-y: auto; */
  }

  .checkbox-item {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  input[type="checkbox"] {
    /* width: 15px;
    height: 15px; */
    width: 25px !important;
    height: 15px !important;
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

  <div class=top-buttons-container>
    <button class="button {filterActive ? 'active' : ''}" on:click={toggleFilter}>
      <Filter /> 
    </button>
    
    <div class="filter-button-container">

      {#if filterActive}
        <div class="filters" transition:slide={{ duration: 300 }}> 
          
          <div class="sent-filter-container">
            <label for="sentFilter">Sent</label>
            <select id="sentFilter" bind:value={sentFilterValue} on:change={() => {
              if (sentFilterValue === 'true') isSent = true;
              else if (sentFilterValue === 'false') isSent = false;
              else isSent = null;
            }}>
              <option value="all">All</option>
              <option value="true">Sent</option>
              <option value="false">Not Sent</option>
            </select>
          </div>

          <label for="checkbox-container">Grade</label>
          <div class="filter-item">
            <div class="checkbox-container">
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
  </div>

  <button class="button" on:click={navigateToNewProject}>
    <PlusCircle /> 
  </button>

  <div class="divider"></div>

  <div class="project-components-container">
    {#each $projectsList as project (project._id)}
      <ProjectComponent {project} on:projectDeleted={() => fetchActiveFilteredProjects()} />
    {/each}
  </div>
</div>
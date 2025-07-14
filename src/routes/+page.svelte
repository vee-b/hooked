<!-- src/routes/inactiveProjects/+page.svelte -->

<script lang="ts">
  // SvelteKit lifecycle and routing tools
  import { onMount, tick } from 'svelte';
  import { goto, afterNavigate } from '$app/navigation';
  
  // Svelte component and utilities
  import ProjectComponent from '../components/ProjectComponent.svelte';
  import { writable } from 'svelte/store';

  // Icons from lucide-svelte
  import { PlusCircle, Filter } from 'lucide-svelte';

  // Type definitions
  import type { Project } from '../models/Project';

  // Project fetching functions (active and filtered)
  import { fetchActiveProjects, fetchActiveFilteredProjects } from '../stores/projectsList';

  // Auth check
  import { checkLoginStatus } from '../controllers/accountsController';

  // Grade-related utilities and filter options
  import { 
    gradeSystem, // reactive store indicating current grade scale ("V-Scale" or "Font Scale")
    getCurrentGrades, // returns the grade list for the selected system
    convertFontScaleGrade, // converts Font scale grades to V-scale (since MongoDB stores in V-scale)
    allStyles, allHolds // preset style and hold tag options 
  } from '../stores/settingsStore';

  import { slide } from 'svelte/transition'

  // Writable store to hold the current project list displayed on the page
  export const projectsList = writable<Project[]>([]);

  // Reactive declaration to watch grade system and fetch the relevant scale (V or Font)
  $: currentGrades = getCurrentGrades($gradeSystem);

  // Navigation button to go to the project creation page
  const navigateToNewProject = () => {
    goto('/projectDetails');
  };

  // Fetch all active projects (no filters)
  const fetchProjects = async () => {
    try {
      const projectsData = await fetchActiveProjects();
      projectsList.set(projectsData);
      console.log('Fetched active projects:', projectsData);
    } catch (error) {
      console.error('Error fetching active projects:', error);
    }
  };

  // Fetch active projects with applied filters
  const fetchFilteredProjects = async (filters: { grades: string[], styles: string[], holds: string[], isSent?: boolean }) => {
    try {
      const isSentParam = filters.isSent !== undefined ? String(filters.isSent) : null;
      const projectsData = await fetchActiveFilteredProjects(filters.grades, String(filters.isSent), filters.styles, filters.holds);
      projectsList.set(projectsData);  // projectsData should be Project[]
      console.log('Fetched projects successfully:', projectsData);
    } catch (error) {
      console.error('Error fetching active projects:', error);
    }
  };

  // When the component first mounts, check login and fetch projects
  onMount(async () => {
    const isLoggedIn = checkLoginStatus();  // Check login status when the component mounts
    if (isLoggedIn) {
      fetchProjects();
    } else {
      goto('/login'); // Redirect if not logged in
    }
  });

  // Refetch projects after every page navigation
  afterNavigate(() => {
    fetchProjects(); 
  });

  // UI State variables for filtering system
  let filterActive = false;
  let selectedGrades: string[] = []; // Store multiple selected grades
  let isSent: boolean | null = null; // null = no filter applied
  let sentFilterValue: string = 'all';
  let selectedStyles: string[] = [];
  let selectedHolds: string[] = [];

  // Toggle the filter panel visibility
  const toggleFilter = () => {
    filterActive = !filterActive;
  };

  // Apply filters and fetch new filtered projects
  const applyFilters = async () => {
    await tick(); // Wait for UI updates
    console.log('Selected Grades:', selectedGrades);  // Log selected grades
    console.log('Selected Styles:', selectedStyles);
    console.log('Selected Holds:', selectedHolds);
    console.log('Applying Filters:', { selectedGrades, isSent });

    // Convert font-scale grades to V-scale before sending to backend
    const gradesToSend = $gradeSystem === 'Font Scale'
      ? selectedGrades.map(grade => convertFontScaleGrade(grade, $gradeSystem))
      : selectedGrades;
    
    const filters = {
      grades: gradesToSend,
      styles: selectedStyles,
      holds: selectedHolds,
      isSent: isSent !== null ? isSent : undefined // omit if null
    };

    console.log('Filters Object:', filters);
    fetchFilteredProjects(filters);
  };

  // Clear all filters and show unfiltered project list
  const clearFilters = async () => {
    selectedGrades = [];
    selectedStyles = [];
    selectedHolds = [];
    isSent = null;
    await tick(); // Ensure UI updates before fetching
    fetchProjects(); // Fetch unfiltered active projects
  }
</script>

<style global>
  body {
    font-family: 'Poppins', sans-serif;
    margin: 0;
    padding: 0;
    color: black;
    margin-bottom: 3rem;
  }

  .home {
    text-align: center;
    padding: 1rem;
    color: black;
    margin-bottom: 3rem;
  }

  .header-container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 2rem;
    position: relative;
    z-index: 10;
    flex-wrap: wrap;
    gap: 1rem;
}

  .button {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem; /* spacing between icon and label */
    padding: 0.75rem 1rem;
    border: none;
    border-radius: 10px;
    font-size: 1rem;
    cursor: pointer;
    background: #ffffff;
    max-width: none; /* allow full content */
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.1), -5px -5px 10px #ffffff;
    transition: box-shadow 0.2s ease;
  }

  .button:hover {
    box-shadow: inset 3px 3px 6px rgba(0, 0, 0, 0.123), inset -3px -3px 6px #ffffff;
  }

  .top-buttons-container {
    display: flex;
    gap: 10px;
    justify-content: center;
    margin-bottom: 1rem;
  }

  .filter-button-container {
    background: #ffffff;
    border-radius: 1rem;
    padding: 1rem;
    margin: 1.5rem auto;
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.1), -5px -5px 10px #ffffff;
    font-family: 'Inter', sans-serif;
    max-width: 90%;
    transition: all 0.3s ease;

    max-width: 600px;
    width: 60%;
    margin: 1rem auto;
  }

  .divider {
    height: 10px;
    margin: 20px 0;
    border-top: 1px solid #ccc;
  }

  .title {
    color: rgb(57, 57, 57);
    font-size: 2rem;
    letter-spacing: 8px;
    text-align: start;
    margin-bottom: 20px;
  }

  .filters {
    display: flex;
    flex-direction: column;
    max-width: 100%;
  }

  .sent-filter-container {
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    align-items: center;
  }

  .sent-filter-container label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #333;
    white-space: nowrap;
    min-width: 50px; /* so label left-align matches */
  }

  .sent-filter-container select {
    padding: 6px 10px;
    font-size: 0.875rem;
    border-radius: 10px;
    border: 1px solid #ccc;
    background: #ffffff;
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.05), -5px -5px 10px #ffffff;
    cursor: pointer;
    min-width: 120px;
    transition: all 0.3s ease;
  }

  .sent-filter-container select:hover {
    box-shadow: inset 3px 3px 6px rgba(0, 0, 0, 0.1), inset -3px -3px 6px #ffffff;
  }

  .filter-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
    margin-bottom: 20px;
  }

  .filter-item label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #333;
    white-space: nowrap;
    margin-left: 8px;
  }

  .filter-button-div {
    display: flex;
    justify-content: center;
    gap: 1rem;
    margin-top: 1rem;
  }

  .filter-button-div .button {
    background: #ffffff;
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.1), -5px -5px 10px #ffffff;
    transition: all 0.3s ease;
    border-radius: 1rem;
  }

  .filter-button-div .button:hover {
    box-shadow: inset 3px 3px 6px rgba(0, 0, 0, 0.1), inset -3px -3px 6px #ffffff;
  }

  select {
    padding: 6px 8px;
    font-size: 0.875rem;
    width: auto;
    border-radius: 6px;
    border: 1px solid #ccc;
  }

  select[multiple] {
    height: 150px;
  }

  .checkbox-container {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    padding: 0.25rem 0;
    font-size: 0.75rem;
    max-height: 200px;
    overflow-y: auto;
  }

  .checkbox-item {
    display: flex;
    align-items: center;
    gap: 4px;
    min-width: 120px;
    margin-bottom: 4px;
  }

  .checkbox-item label {
    width: 50px; /* Ensure equal width for alignment */
    text-align: left;
  }

  input[type="checkbox"] {
    width: 25px !important;
    height: 15px !important;
  }

  .error-message {
    color: red;
    font-size: 0.875rem;
    margin-bottom: 10px;
  }
</style>

<!-- MAIN PAGE LAYOUT -->
<div class="home">

  <!-- Header Section -->
  <div class="header-container">
    <h1 class="title">Active Projects</h1>
  </div>

  <div class="divider"></div>

  <!-- Top action buttons: Toggle Filter | New Project -->
  <div class="top-buttons-container">
    <button class="button {filterActive ? 'active' : ''}" on:click={toggleFilter}>
      <Filter size={18}/>
      <!-- <span>Filters</span> -->
    </button>
    <button class="button" on:click={navigateToNewProject}>
      <PlusCircle size={18}/>
      <!-- <span>New Project</span> -->
    </button>
  </div>

  <!-- Filter Panel (Only visible if filterActive === true) -->
  {#if filterActive}
    <div class="filter-button-container" transition:slide={{ duration: 300 }}>
      <div class="filters">
        <!-- Sent Status Dropdown -->
        <div class="sent-filter-container">
          <label for="sentFilter">Sent</label>
          <select id="sentFilter" bind:value={sentFilterValue} on:change={() => {
            isSent = sentFilterValue === 'true' ? true : sentFilterValue === 'false' ? false : null;
          }}>
            <option value="all">All</option>
            <option value="true">Sent</option>
            <option value="false">Not Sent</option>
          </select>
        </div>

        <!-- Grade Checkboxes -->
        <div class="filter-item">
          <label>Grades</label>
          <div class="checkbox-container">
            {#each currentGrades as grade}
              <div class="checkbox-item">
                <input
                  type="checkbox"
                  bind:group={selectedGrades}
                  value={grade}
                  id={grade}
                />
                <label for={grade}>{grade}</label>
              </div>
            {/each}
          </div>
        </div>

        <!-- Style Checkboxes -->
        <div class="filter-item">
          <label>Styles</label>
          <div class="checkbox-container">
            {#each allStyles as style}
              <div class="checkbox-item">
                <input
                  type="checkbox"
                  bind:group={selectedStyles}
                  value={style}
                  id={style}
                />
                <label for={style}>{style}</label>
              </div>
            {/each}
          </div>
        </div>

        <!-- Hold Type Checkboxes -->
        <div class="filter-item">
          <label>Holds</label>
          <div class="checkbox-container">
            {#each allHolds as hold}
              <div class="checkbox-item">
                <input
                  type="checkbox"
                  bind:group={selectedHolds}
                  value={hold}
                  id={hold}
                />
                <label for={hold}>{hold}</label>
              </div>
            {/each}
          </div>
        </div>

        <!-- Apply / Clear Buttons -->
        <div class="filter-button-div">
          <button class="button" on:click={applyFilters}>Apply</button>
          <button class="button" on:click={clearFilters}>Clear</button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Project List Section -->
  <div class="project-components-container">
    {#each $projectsList as project (project._id)}
      <ProjectComponent {project} />
    {/each}
  </div>
</div>

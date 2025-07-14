<!-- src/routes/inactiveProjects/+page.svelte -->

<script lang="ts">
  // --- SVELTE + ROUTING TOOLS ---
  import { onMount, tick } from 'svelte';
  import { goto, afterNavigate } from '$app/navigation';

  // --- UI COMPONENTS & ICONS ---
  import ProjectComponent from '../../components/ProjectComponent.svelte';
  import { PlusCircle, Filter } from 'lucide-svelte';

  // --- STATE MANAGEMENT ---
  import { writable } from 'svelte/store';

  // --- TYPES ---
  import type { Project } from '../../models/Project';

  // --- PROJECT FETCHING FUNCTIONS ---
  import { 
    fetchInactiveProjects, // gets ALL inactive projects
    fetchInactiveFilteredProjects, // gets inactive projects based on filters
    deleteProject // deletes a project (called by child component)
  } from '../../stores/projectsList';

  // --- AUTH + SETTINGS ---
  import { checkLoginStatus } from '../../controllers/accountsController';
  
  // --- TRANSITIONS ---
  import { slide } from 'svelte/transition'

  // --- GRADES & FILTER OPTIONS ---
  import { gradeSystem, getCurrentGrades, convertFontScaleGrade, allStyles, allHolds } from '../../stores/settingsStore';
  
  // --- STATE VARIABLES ---
  export const projectsList = writable<Project[]>([]); // holds the list of inactive projects for display

  // re-compute currentGrades when $gradeSystem changes (reactive declaration)
  $: currentGrades = getCurrentGrades($gradeSystem);
  
  // --- DATA FETCHING FUNCTIONS ---

  // Fetch all inactive projects (no filters applied)
  const fetchProjects = async () => {
    try {
      const projectsData = await fetchInactiveProjects();
      projectsList.set(projectsData);
      console.log('Fetched active projects:', projectsData);

    } catch (error) {
      console.error('Error fetching active projects:', error);
    }
  };
  
  // Fetch inactive projects with filtering applied
  const fetchFilteredProjects = async (filters: { grades: string[], styles: string[], holds: string[], isSent?: boolean }) => {
    try {
      const isSentParam = filters.isSent !== undefined ? String(filters.isSent) : null;
      const projectsData = await fetchInactiveFilteredProjects(filters.grades, String(filters.isSent), filters.styles, filters.holds);
      projectsList.set(projectsData);  // projectsData should be Project[]
      console.log('Fetched projects successfully:', projectsData);
    } catch (error) {
      console.error('Error fetching active projects:', error);
    }
  };
  
  // --- ON COMPONENT LOAD ---
  onMount(() => {
    const isLoggedIn = checkLoginStatus();  // Check login status when the component mounts
    if (isLoggedIn) {
      fetchProjects();
    } else {
      goto('/login'); // Redirect if not logged in
    }
  });

  // Refresh projects on any route change
  afterNavigate(() => {
    fetchProjects(); // or fetchFilteredProjects if filters should persist
  });
  
  // --- FILTER CONTROL VARIABLES ---
  let filterActive = false; // toggles the visibility of the filter UI panel
  let selectedGrades: string[] = []; // stores selected grades from checkboxes
  let isSent: boolean | null = null; // stores sent filter (true, false or null=all)
  let sentFilterValue: string = 'all'; // actual dropdown string state
  let selectedStyles: string[] = []; // selected climbing styles
  let selectedHolds: string[] = []; // selected hold types
    
  // --- FILTER FUNCTIONS ---
  const toggleFilter = () => {
    filterActive = !filterActive;
  };
  
  const applyFilters = async () => {
    await tick(); // ensure UI updates before data fetch
    console.log('Applying Filters:', { selectedGrades, selectedStyles, selectedHolds, isSent });
      
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
  
  const clearFilters = async () => {
    selectedGrades = [];
    selectedStyles = [];
    selectedHolds = [];
    isSent = null;
    await tick(); // Wait for UI to update
    fetchProjects(); // Reset to unfiltered list
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

.title {
  color: rgb(57, 57, 57);
  font-size: 2rem;
  letter-spacing: 8px;
  margin: 0;
  margin: 2rem 0 1rem;
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
    flex-direction: column;
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
  
  <!-- Page Header -->
  <div class="header-container">
    <h1 class="title">Inactive Projects</h1>
  </div>

  <div class="divider"></div>

  <!-- Top Action Buttons -->
  <div class=top-buttons-container>
    <button class="button {filterActive ? 'active' : ''}" on:click={toggleFilter}>
      <Filter size={18}/>
    </button>
  </div>
    
  <!-- Filter Panel -->
    {#if filterActive}
    <div class="filter-button-container" transition:slide={{ duration: 300 }}>
      <div class="filters">
        <!-- Filter by Sent Status -->
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

        <!-- Filter by Grades -->
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

        <!-- Filter by Styles -->
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

        <!-- Filter by Holds -->
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

  <!-- Project List -->
  <div class="project-components-container">
    {#each $projectsList as project (project._id)}
      <ProjectComponent {project} on:projectDeleted={() => fetchInactiveFilteredProjects()} />
    {/each}
  </div>
</div>
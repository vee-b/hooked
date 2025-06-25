<!-- src/routes/inactiveProjects/+page.svelte -->

<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import ProjectComponent from '../../components/ProjectComponent.svelte';
  import { writable } from 'svelte/store';
  import { PlusCircle, Filter } from 'lucide-svelte';
  import type { Project } from '../../models/Project';
  import { fetchInactiveProjects, fetchInactiveFilteredProjects, deleteProject } from '../../stores/projectsList';
  import { checkLoginStatus, logoutAccount } from '../../controllers/accountsController';
  import { tick } from 'svelte';
  import { slide } from 'svelte/transition'
  import { afterNavigate } from '$app/navigation';
  import { gradeSystem, getCurrentGrades, convertFontScaleGrade } from '../../stores/settingsStore';
  
  export const projectsList = writable<Project[]>([]);

  // Grade conversion
  $: currentGrades = getCurrentGrades($gradeSystem);
  
  const fetchProjects = async () => {
  try {
    const projectsData = await fetchInactiveProjects();
    projectsList.set(projectsData);
    console.log('Fetched active projects:', projectsData);
  } catch (error) {
    console.error('Error fetching active projects:', error);
  }
};
  
  const fetchFilteredProjects = async (filters: { grades: string[], isSent?: boolean }) => {
    try {
      const isSentParam = filters.isSent !== undefined ? String(filters.isSent) : null;
      const projectsData = await fetchInactiveFilteredProjects(filters.grades, String(filters.isSent));
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

  afterNavigate(() => {
    fetchProjects(); // or fetchFilteredProjects if filters should persist
  });
  
  let filterActive = false;
  let selectedGrades: string[] = []; // Store multiple selected grades
  let isSent: boolean | null = null; // null = no filter applied
  let sentFilterValue: string = 'all';
    
  const toggleFilter = () => {
    filterActive = !filterActive;
  };
  
  const applyFilters = async () => {
    await tick(); // Wait for UI updates
    console.log('Selected Grades:', selectedGrades);  // Log selected grades
    console.log('Applying Filters:', { selectedGrades, isSent });
      
    const gradesToSend = $gradeSystem === 'Font Scale'
      ? selectedGrades.map(grade => convertFontScaleGrade(grade, $gradeSystem))
      : selectedGrades;
    
    const filters = {
      grades: gradesToSend,
      isSent: isSent !== null ? isSent : undefined // omit if null
    };
  
    console.log('Filters Object:', filters);
    fetchFilteredProjects(filters);
  };
  
  const clearFilters = async () => {
    selectedGrades = [];
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
  
<div class="home">
  
  <div class="header-container">
    <h1 class="title">Inactive Projects</h1>
    <!-- <button class="button logout-button" on:click={handleLogout}>
      Logout
    </button> -->
  </div>

  <div class="divider"></div>

  <div class=top-buttons-container>
    <button class="button {filterActive ? 'active' : ''}" on:click={toggleFilter}>
      <Filter size={18}/>
      <span>Filters</span>
    </button>
  </div>
    
    {#if filterActive}
    <div class="filter-button-container" transition:slide={{ duration: 300 }}>
      <div class="filters">
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

        <div class="filter-button-div">
          <button class="button" on:click={applyFilters}>Apply</button>
          <button class="button" on:click={clearFilters}>Clear</button>
        </div>
      </div>
    </div>
  {/if}

  <div class="project-components-container">
    {#each $projectsList as project (project._id)}
      <ProjectComponent {project} on:projectDeleted={() => fetchInactiveFilteredProjects()} />
    {/each}
  </div>
</div>
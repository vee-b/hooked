<script>
    import { onMount } from 'svelte';
    import { writable } from 'svelte/store';

    // Define the Project type for JSDoc
    /**
     * @typedef {Object} Project
     * @property {boolean} isSent
     * @property {string} grade
     */

    // Explicitly define the type for the projectsList prop to accept Writable<Project[]>
    /** 
     * @type {import('svelte/store').Writable<Project[]>}
     */
     export let projectsList;
  
    // Create a writable store to hold sends count
    const sendsCount = writable(/** @type {Record<string, number | undefined>} */ ({}));
  
    // Function to fetch sends count for a specific grade
    /**
     * @param {string} grade
     */
   async function fetchSendsCount(grade) {
    try {
        // Get the current projects from the store by subscribing to it
        /**
        * @type {Project[]}
        */
        let projects = [];
        const unsubscribe = projectsList.subscribe(value => {
        projects = value;
    });

    // Use the unsubscribe function to clean up the subscription
    unsubscribe();

    // Check if projects is defined and filter the projects to count sends by grade
    if (projects) {
        // @ts-ignore
        const count = projects.filter(project => project.isSent && project.grade === grade).length;
        sendsCount.update(counts => ({ ...counts, [grade]: count }));
        } else {
            console.error('No projects found');
            }
        } catch (error) {
            console.error('Error fetching sends count:', error);
        }
    }
  
    // On mount, initialize sends count for each grade
    onMount(() => {
      const grades = ['V0', 'V1', 'V2', 'V3', 'V4', 'V5', 'V6', 'V7', 'V8', 'V9', 'V10'];
      grades.forEach(grade => {
        fetchSendsCount(grade);
      });
    });
  </script>
  
  <style>
    .sends-card {
      margin: 16px;
      padding: 12px;
      border: 1px solid #ccc;
      border-radius: 4px;
    }
  
    .sends-text {
      font-size: 18px;
      color: gray;
    }
  </style>
  
  <div class="sends-card">
    {#each ['V0', 'V1', 'V2', 'V3', 'V4', 'V5', 'V6', 'V7', 'V8', 'V9', 'V10'] as grade}
      <div class="sends-text">
        {$sendsCount[grade] !== undefined
          ? `${grade} Sends: ${$sendsCount[grade]}`
          : 'Loading...'}
      </div>
    {/each}
  </div>

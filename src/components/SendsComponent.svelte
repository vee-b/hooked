<script >
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';
  import { sendsSummary, fetchSendsSummary } from '../stores/projectsList';
  import { Project } from '../models/Project'; // Import the Project type from the same module
  
  const grades = ['V0', 'V1', 'V2', 'V3', 'V4', 'V5', 'V6', 'V7', 'V8', 'V9', 'V10', 'V11', 'V12', 'V13', 'V14', 'V15', 'V16', 'V17'];

  // On mount, initialize sends count for each grade
  onMount(async () => {
    console.log('Fetching sends summary...');
    await fetchSendsSummary(); // Now manages the sends summary internally, no need to pass projectsList
    console.log('Sends summary fetched:', $sendsSummary);
  });
</script>
  
<style>
  .sends-card {
    margin: 16px;
    padding: 12px;
    border: 1px solid #ccc;
    border-radius: 4px;
  }

  .sends-total {
    font-size: 18px;
    color: gray;
    margin-bottom: 16px;
  }
  
  .sends-text {
    font-size: 18px;
    color: gray;
  }
</style>
  
<div class="sends-card">
  <div class="sends-total">
    Total Sends: {$sendsSummary.total} 
  </div>

  {#each grades as grade}
    <div class="sends-text">
      {#if $sendsSummary.byGrade[grade] !== undefined}
        {grade} Sends: {$sendsSummary.byGrade[grade]}
      {:else}
        {grade} Sends: 0
      {/if}
    </div>
  {/each}
</div>
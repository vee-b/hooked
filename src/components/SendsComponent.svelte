<script >
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';
  import { sendsSummary, fetchSendsSummary } from '../stores/projectsList';
  import { Project } from '../models/Project'; // Import the Project type from the same module
  import { convertVScaleGrade, vScale, fontScale, gradeSystem } from '../stores/settingsStore'
  
  // On mount, initialize sends count for each grade
  onMount(async () => {
    console.log('Fetching sends summary...');
    await fetchSendsSummary(); // Now manages the sends summary internally, no need to pass projectsList
    console.log('Sends summary fetched:', $sendsSummary);
  });
</script>

<style>
  .sends-card {
    font-family: 'Poppins', sans-serif;
    padding: 1.5rem;
    margin: 1rem 0;
    background: #ffffff;
    border-radius: 12px;
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.05), -5px -5px 10px #ffffff;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .sends-total {
    font-size: 1.25rem;
    font-weight: 600;
    color: rgb(57, 57, 57);
    letter-spacing: 1px;
  }

  .sends-text {
    font-size: 1rem;
    color: rgb(90, 90, 90);
  }
</style>

<div class="sends-card">
  <div class="sends-total">
    Total Sends: {$sendsSummary.total}
  </div>

  {#each vScale as grade}
    <div class="sends-text">
      {convertVScaleGrade(grade, $gradeSystem)} Sends: {$sendsSummary.byGrade[grade] ?? 0}
    </div>
  {/each}
</div>

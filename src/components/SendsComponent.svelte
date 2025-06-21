<!-- <script >
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
</script> -->

<!-- <style>
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
</style> -->

<!-- <div class="sends-card">
  <div class="sends-total">
    Total Sends: {$sendsSummary.total}
  </div>

  {#each vScale as grade}
    <div class="sends-text">
      {convertVScaleGrade(grade, $gradeSystem)} Sends: {$sendsSummary.byGrade[grade] ?? 0}
    </div>
  {/each}
</div> -->

<script>
  import { onMount } from 'svelte';
  import { sendsSummary, fetchSendsSummary } from '../stores/projectsList';
  import { convertVScaleGrade, vScale, gradeSystem } from '../stores/settingsStore';

  let maxSends = 1;

  onMount(async () => {
    await fetchSendsSummary();
    const values = Object.values($sendsSummary.byGrade ?? {});
    maxSends = Math.max(...values, 1); // prevent divide-by-zero
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
    margin-bottom: 1rem;
  }

  .grade-row {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .grade-label {
    font-weight: bold;
    font-size: 1rem;
    width: 60px;
    text-align: right;
  }

  .progress-bar {
    flex: 1;
    height: 1.2rem;
    background-color: #f0f0f0;
    border-radius: 8px;
    overflow: hidden;
    position: relative;
  }

  .progress-fill {
    height: 100%;
    background-color: #009fb7;
    transition: width 0.3s ease;
    border-radius: 8px 0 0 8px;
  }

  .send-count {
    font-size: 0.9rem;
    width: 30px;
    text-align: left;
    color: rgb(90, 90, 90);
  }
</style>

<div class="sends-card">
  <div class="sends-total">
    Total Sends: {$sendsSummary.total}
  </div>

  {#each vScale as grade}
    <div class="grade-row">
      <div class="grade-label">
        {convertVScaleGrade(grade, $gradeSystem)}
      </div>
      <div class="progress-bar">
        <div
          class="progress-fill"
          style="width: {Math.round((($sendsSummary.byGrade[grade] ?? 0) / maxSends) * 100)}%"
        ></div>
      </div>
      <div class="send-count">
        {$sendsSummary.byGrade[grade] ?? 0}
      </div>
    </div>
  {/each}
</div>

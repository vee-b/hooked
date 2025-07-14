<!-- src/components/SendsComponent.svelte -->

<!-- 
Dynamic bar visualization driven by a normalized reactive data model:

This component implements a dynamic visualization of climbing project completions (sends) by grade, rendered as proportional progress bars. The approach leverages Svelte’s reactive store paradigm to bind data updates directly to the user interface without imperative DOM manipulation.

The data model is organized in a sendsSummary store, which maintains:

total: an aggregated count of all sends.

byGrade: a dictionary mapping each grade (e.g., V0, V1, ...) to its respective send count.

Upon component initialization (onMount), the application asynchronously retrieves this summary via fetchSendsSummary, ensuring that the visualization reflects the latest state. A normalization step computes the maximum send count across all grades (maxSends), which prevents division by zero and standardizes the scaling of each progress bar.

Each grade's relative completion level is calculated as:

'''
width percentage = (sends for grade / max sends across grades) x 100
'''

This percentage is then directly bound to the width of the .progress-fill div. As a result, any mutation to the sendsSummary store—whether from user actions or external data updates—automatically reflows the bar lengths due to Svelte's reactive re-evaluation.

This pattern exemplifies a normalized reactive data visualization pipeline, promoting scalability and maintainability by separating data acquisition, transformation (normalization), and presentation (progress bars). 
-->

<script>
  import { onMount } from 'svelte';
  import { sendsSummary, fetchSendsSummary } from '../stores/projectsList';
  import { convertVScaleGrade, vScale, gradeSystem } from '../stores/settingsStore';

  // VARIABLES & STATE
  // Holds the maximum number of sends among all grades
  // (used to normalize progress bars to % width)
  let maxSends = 1;

  // ON MOUNT: fetch summary stats from store
  onMount(async () => {
    // This will populate the `sendsSummary` store with data from backend
    await fetchSendsSummary();

    // Extract an array of sends counts like [3, 5, 0, 2] from byGrade object
    const values = Object.values($sendsSummary.byGrade ?? {});

    // Compute the highest send count. 
    // We also pass 1 to avoid divide-by-zero (so denominator is never zero)
    maxSends = Math.max(...values, 1); 
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
    gap: 0.75rem;
  }

  .grade-label {
    font-weight: bold;
    font-size: 1rem;
    min-width: 50px;
    text-align: left;
  }

  .progress-bar {
    flex: 1;
    height: 0.6rem;
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
  <!-- Displays the total sends across all grades -->
  <div class="sends-total">
    Total Sends: {$sendsSummary.total}
  </div>

  <!-- Iterate through each grade in the vScale array -->
  {#each vScale as grade}
    <div class="grade-row">
      <!-- Show the grade label (converted to user's preferred system) -->
      <div class="grade-label">
        {convertVScaleGrade(grade, $gradeSystem)}
      </div>

      <!-- Progress bar that visually shows the sends for this grade -->
      <div class="progress-bar">
        <div
          class="progress-fill"
          style="width: {Math.round((($sendsSummary.byGrade[grade] ?? 0) / maxSends) * 100)}%"
        ></div>
      </div>

      <!-- Shows the raw count next to the bar -->
      <div class="send-count">
        {$sendsSummary.byGrade[grade] ?? 0}
      </div>
    </div>
  {/each}
</div>

<!-- 
                 +----------------------+
                 |   fetchSendsSummary  |
                 |   (onMount)          |
                 +----------------------+
                            |
                            v
             +--------------------------------+
             |  sendsSummary Svelte store     |
             |  {                             |
             |    total: 35,                  |
             |    byGrade: { V0: 4, V1: 8...} |
             |  }                             |
             +--------------------------------+
                            |
         +------------------+------------------+
         |                                     |
         v                                     v
+--------------------+              +------------------------+
| compute maxSends   |              |  convertVScaleGrade    |
| = max(byGrade...)  |              | (V0 -> Font if needed) |
+--------------------+              +------------------------+
         |                                     |
         +------------------+------------------+
                            v
             +--------------------------------+
             |   Svelte {#each vScale} loop   |
             |   For each grade:              |
             |    - compute % width           |
             |    - render progress bar       |
             +--------------------------------+
                            |
                            v
             +--------------------------------+
             |    DOM updates with bar widths |
             |    and grade labels            |
             +--------------------------------+

-->
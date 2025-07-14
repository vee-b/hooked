<!-- src/components/StylesRadarGraphComponent.svelte -->

<script lang="ts">
  import { onMount } from 'svelte';
  import { Chart } from 'chart.js/auto';
  import { stylesSummary, fetchStylesSummary } from '../stores/projectsList';
  import { allStyles } from '../stores/settingsStore';
  import { writable } from 'svelte/store';

  // Reference to the <canvas> element where we render the radar chart
  let canvasEl: HTMLCanvasElement | null = null;
  // Holds the Chart.js radar chart instance so we can update it later
  let radarChart: Chart | null = null;

  // Toggle state to switch between counts vs percentages
  let showPercent = false;

  // ON MOUNT: fetch styles summary data from the store
  onMount(async () => {
    await fetchStylesSummary();
  });

  // COMPUTED DERIVED DATA
  // This creates a consistent array matching allStyles, combining the fetched summary
  $: styleChartData = allStyles.map(style => {
    const found = $stylesSummary.find(item => item.style === style);
    return {
      style,
      done: found?.done ?? 0,
      practicing: found?.practicing ?? 0
    };
  });

  // Totals for calculating percentages
  $: totalDone = styleChartData.reduce((sum, item) => sum + item.done, 0);
  $: totalPracticing = styleChartData.reduce((sum, item) => sum + item.practicing, 0);

  // Compute percentage data relative to totals
  $: stylePercentData = styleChartData.map(item => ({
    style: item.style,
    done: totalDone ? +((item.done / totalDone) * 100).toFixed(2) : 0,
    practicing: totalPracticing ? +((item.practicing / totalPracticing) * 100).toFixed(2) : 0
  }));

  // WATCHER: whenever data changes or the toggle changes,
  // rebuild or update the radar chart
  $: if (canvasEl && styleChartData.length) {
    const labels = styleChartData.map(item => item.style);
    const dataDone = showPercent 
      ? stylePercentData.map(item => item.done)
      : styleChartData.map(item => item.done);
    const dataPracticing = showPercent 
      ? stylePercentData.map(item => item.practicing)
      : styleChartData.map(item => item.practicing);

    if (!radarChart) {
      // INITIALIZE A NEW RADAR CHART
      radarChart = new Chart(canvasEl, {
        type: 'radar',
        data: {
          labels,
          datasets: [
            {
              // label: showPercent ? 'Done (%)' : 'Done',
              label: 'Done',
              data: dataDone,
              backgroundColor: 'rgba(0, 159, 183, 0.3)',
              borderColor: 'rgba(0, 159, 183, 1)',
              pointBackgroundColor: 'rgba(0, 159, 183, 1)',
              fill: true
            },
            {
              // label: showPercent ? 'Practicing (%)' : 'Practicing',
              label: 'Practicing',
              data: dataPracticing,
              backgroundColor: 'rgba(255, 165, 0, 0.3)',
              borderColor: 'rgba(255, 165, 0, 1)',
              pointBackgroundColor: 'rgba(255, 165, 0, 1)',
              fill: true
            }
          ]
        },
        options: {
          responsive: true,
          aspectRatio: 1,
          elements: {
             line: { borderWidth: 2 },
             point: { radius: 2 }
            },
          scales: {
            r: { 
              beginAtZero: true, 
              max: showPercent ? 100 : undefined,
              ticks: {
                stepSize: showPercent ? 20 : 1, 
                font: {
                  size: 8  // make tick labels smaller
                }
                // maxTicksLimit: 6
              }
            }
          }
        }
      });
    } else {
      // UPDATE EXISTING CHART
      radarChart.data.labels = labels;
      radarChart.data.datasets[0].data = dataDone;
      radarChart.data.datasets[0].label = 'Done';
      radarChart.data.datasets[1].data = dataPracticing;
      radarChart.data.datasets[1].label = 'Practicing';
    
      // Also update the scale for percentages
      const rScale = radarChart.options.scales?.r;
      if (rScale?.ticks) {
        rScale.max = showPercent ? 100 : undefined;
        (rScale.ticks as { stepSize?: number }).stepSize = showPercent ? 20 : 1;
      }
      radarChart.update();
    }
  }

  // ACTION: Toggle between showing counts or percentages
  function toggleMode() {
    showPercent = !showPercent;
  }
</script>

<style>
  .toggle-container {
    text-align: center;
    margin-top: 1rem;
  }

  .toggle-button {
    background: #ffffff;
    border-radius: 10px;
    border: 1px solid #ccc;
    padding: 0.5rem 1rem;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.3s ease;
    box-shadow: 3px 3px 6px rgba(0,0,0,0.1), -3px -3px 6px #ffffff;
  }

  .toggle-button:hover {
    box-shadow: inset 2px 2px 4px rgba(0,0,0,0.1), inset -2px -2px 4px #ffffff;
  }

  canvas {
    max-width: 100%;
    width: 100%;
    height: auto;
    margin: 0 auto;
    display: block;
    background: none;
    box-shadow: none;
  }

  .styles-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: rgb(57, 57, 57);
    letter-spacing: 1px;
    margin-bottom: 1rem;
  }
</style>

<div class="styles-radar-container">
  <!-- Title for the chart -->
  <div class="styles-title">Styles</div>

  <!-- The canvas element will bind to `canvasEl` -->
  <canvas bind:this={canvasEl}></canvas>

  <!-- Toggle button to switch between counts and percentages -->
  <div class="toggle-container">
    <button class="toggle-button" on:click={toggleMode}>
      Toggle to {showPercent ? 'Counts' : 'Percentages'}
    </button>
  </div>
</div>

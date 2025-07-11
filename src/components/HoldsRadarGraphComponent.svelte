<script lang="ts">
  import { onMount } from 'svelte';
  import { Chart } from 'chart.js/auto';
  import { holdsSummary, fetchHoldsSummary } from '../stores/projectsList';
  import { allHolds } from '../stores/settingsStore';
  import { writable } from 'svelte/store';

  let canvasEl: HTMLCanvasElement | null = null;
  let radarChart: Chart | null = null;

  // Toggle state
  let showPercent = false;

  onMount(async () => {
    await fetchHoldsSummary();
  });

  // Prepare normalized data
  $: holdsChartData = allHolds.map(holds => {
    const found = $holdsSummary.find(item => item.holds === holds);
    return {
      holds,
      done: found?.done ?? 0,
      practicing: found?.practicing ?? 0
    };
  });

  $: totalDone = holdsChartData.reduce((sum, item) => sum + item.done, 0);
  $: totalPracticing = holdsChartData.reduce((sum, item) => sum + item.practicing, 0);

  // Calculate % data relative to total done/practicing
  $: holdsPercentData = holdsChartData.map(item => ({
    style: item.holds,
    done: totalDone ? +((item.done / totalDone) * 100).toFixed(2) : 0,
    practicing: totalPracticing ? +((item.practicing / totalPracticing) * 100).toFixed(2) : 0
  }));

  // Watch for changes
  $: if (canvasEl && holdsChartData.length) {
    const labels = holdsChartData.map(item => item.holds);
    const dataDone = showPercent 
      ? holdsPercentData.map(item => item.done)
      : holdsChartData.map(item => item.done);
    const dataPracticing = showPercent 
      ? holdsPercentData.map(item => item.practicing)
      : holdsChartData.map(item => item.practicing);

    if (!radarChart) {
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
      radarChart.data.labels = labels;
      radarChart.data.datasets[0].data = dataDone;
      // radarChart.data.datasets[0].label = showPercent ? 'Done (%)' : 'Done';
      radarChart.data.datasets[0].label = 'Done';
      radarChart.data.datasets[1].data = dataPracticing;
      // radarChart.data.datasets[1].label = showPercent ? 'Practicing (%)' : 'Practicing';
      radarChart.data.datasets[1].label = 'Practicing';
    
      const rScale = radarChart.options.scales?.r;
      if (rScale?.ticks) {
        rScale.max = showPercent ? 100 : undefined;
        (rScale.ticks as { stepSize?: number }).stepSize = showPercent ? 20 : 1;
      }
      radarChart.update();
    }
  }

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

  .holds-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: rgb(57, 57, 57);
    letter-spacing: 1px;
    margin-bottom: 1rem;
  }
</style>

<div class="holds-radar-container">
  <div class="holds-title">Holds</div>
  <canvas bind:this={canvasEl}></canvas>
  <div class="toggle-container">
    <button class="toggle-button" on:click={toggleMode}>
      Toggle to {showPercent ? 'Counts' : 'Percentages'}
    </button>
  </div>
</div>

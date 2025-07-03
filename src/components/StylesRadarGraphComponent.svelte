<!-- <script lang="ts">
  import { onMount } from 'svelte';
  import { Chart } from 'chart.js/auto';
  import { stylesSummary, fetchStylesSummary } from '../stores/projectsList';
  import { allStyles } from '../stores/settingsStore';

  let canvasEl: HTMLCanvasElement | null = null;
  let radarChart: Chart | null = null;

  onMount(async () => {
    await fetchStylesSummary();
  });

  // Normalize data: ensure each style in allStyles has data
  $: styleChartData = allStyles.map(style => {
    const found = $stylesSummary.find(item => item.style === style);
    return {
      style,
      done: found?.done ?? 0,
      practicing: found?.practicing ?? 0
    };
  });

  // Watch data + canvas to build or update chart
  $: if (canvasEl && styleChartData.length) {
    if (!radarChart) {
      radarChart = new Chart(canvasEl, {
        type: 'radar',
        data: {
          labels: styleChartData.map(item => item.style),
          datasets: [
            {
                label: 'Done',
                data: styleChartData.map(item => item.done),
                backgroundColor: 'rgba(0, 159, 183, 0.3)', // light teal
                borderColor: 'rgba(0, 159, 183, 1)',       // dark teal
                pointBackgroundColor: 'rgba(0, 159, 183, 1)',
                fill: true
            },
            {
                label: 'Practicing',
                data: styleChartData.map(item => item.practicing),
                backgroundColor: 'rgba(255, 165, 0, 0.3)',  // orange fill
                borderColor: 'rgba(255, 165, 0, 1)',        // orange line
                pointBackgroundColor: 'rgba(255, 165, 0, 1)',
                fill: true
            }
          ]
        },
        options: {
          responsive: true,
          elements: { line: { borderWidth: 2 }},
          scales: {
            r: { beginAtZero: true, ticks: { stepSize: 1 }}
          }
        }
      });
    } else {
      // update existing chart
      radarChart.data.labels = styleChartData.map(item => item.style);
      radarChart.data.datasets[0].data = styleChartData.map(item => item.done);
      radarChart.data.datasets[1].data = styleChartData.map(item => item.practicing);
      radarChart.update();
    }
  }
</script>

<style>
  canvas {
    max-width: 100%;
    width: 100%;
    height: auto;
    margin: 0 auto;
    display: block;
    background: none;
    box-shadow: none;
  }
</style>

<div class="styles-radar-container">
  <canvas bind:this={canvasEl}></canvas>
</div> -->

<script lang="ts">
  import { onMount } from 'svelte';
  import { Chart } from 'chart.js/auto';
  import { stylesSummary, fetchStylesSummary } from '../stores/projectsList';
  import { allStyles } from '../stores/settingsStore';
  import { writable } from 'svelte/store';

  let canvasEl: HTMLCanvasElement | null = null;
  let radarChart: Chart | null = null;

  // Toggle state
  let showPercent = false;

  onMount(async () => {
    await fetchStylesSummary();
  });

  // Prepare normalized data
  $: styleChartData = allStyles.map(style => {
    const found = $stylesSummary.find(item => item.style === style);
    return {
      style,
      done: found?.done ?? 0,
      practicing: found?.practicing ?? 0
    };
  });

  // Calculate % data
  $: stylePercentData = styleChartData.map(item => {
    const total = item.done + item.practicing;
    return {
      style: item.style,
      done: total ? +( (item.done / total) * 100 ).toFixed(2) : 0,
      practicing: total ? +( (item.practicing / total) * 100 ).toFixed(2) : 0
    };
  });

  // Watch for changes
  $: if (canvasEl && styleChartData.length) {
    const labels = styleChartData.map(item => item.style);
    const dataDone = showPercent 
      ? stylePercentData.map(item => item.done)
      : styleChartData.map(item => item.done);
    const dataPracticing = showPercent 
      ? stylePercentData.map(item => item.practicing)
      : styleChartData.map(item => item.practicing);

    if (!radarChart) {
      radarChart = new Chart(canvasEl, {
        type: 'radar',
        data: {
          labels,
          datasets: [
            {
              label: showPercent ? 'Done (%)' : 'Done',
              data: dataDone,
              backgroundColor: 'rgba(0, 159, 183, 0.3)',
              borderColor: 'rgba(0, 159, 183, 1)',
              pointBackgroundColor: 'rgba(0, 159, 183, 1)',
              fill: true
            },
            {
              label: showPercent ? 'Practicing (%)' : 'Practicing',
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
          elements: { line: { borderWidth: 2 }},
          scales: {
            r: { 
              beginAtZero: true, 
              max: showPercent ? 100 : undefined,
              ticks: { stepSize: showPercent ? 20 : 1 }
            }
          }
        }
      });
    } else {
      radarChart.data.labels = labels;
      radarChart.data.datasets[0].data = dataDone;
      radarChart.data.datasets[0].label = showPercent ? 'Done (%)' : 'Done';
      radarChart.data.datasets[1].data = dataPracticing;
      radarChart.data.datasets[1].label = showPercent ? 'Practicing (%)' : 'Practicing';
      
      // if (radarChart.options.scales?.r) {
      //   radarChart.options.scales.r.max = showPercent ? 100 : undefined;
      //   radarChart.options.scales.r.ticks.stepSize = showPercent ? 20 : 1;
      // }
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
</style>

<div class="styles-radar-container">
  <canvas bind:this={canvasEl}></canvas>
  <div class="toggle-container">
    <button class="toggle-button" on:click={toggleMode}>
      Toggle to {showPercent ? 'Counts' : 'Percentages'}
    </button>
  </div>
</div>

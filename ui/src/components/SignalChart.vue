<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { Line } from "vue-chartjs";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

const chartData = ref({
  labels: [] as string[],
  datasets: [
    {
      label: "Vehicle Speed (km/h)",
      backgroundColor: "#38bdf8",
      borderColor: "#38bdf8",
      data: [] as number[],
      tension: 0.4,
      pointRadius: 0,
    },
  ],
});

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    y: {
      beginAtZero: true,
      grid: { color: "rgba(148, 163, 184, 0.1)" },
      ticks: { color: "#94a3b8" },
    },
    x: {
      grid: { display: false },
      ticks: { display: false },
    },
  },
  plugins: {
    legend: { display: false },
  },
  animation: { duration: 0 },
};

let interval: any;

onMounted(() => {
  let count = 0;
  interval = setInterval(() => {
    const time = new Date().toLocaleTimeString();
    const val = 60 + Math.random() * 20;

    chartData.value.labels.push(time);
    chartData.value.datasets[0].data.push(val);

    if (chartData.value.labels.length > 50) {
      chartData.value.labels.shift();
      chartData.value.datasets[0].data.shift();
    }

    // Trigger update
    chartData.value = { ...chartData.value };
    count++;
  }, 200);
});

onUnmounted(() => {
  clearInterval(interval);
});
</script>

<template>
  <div style="height: 300px; width: 100%">
    <Line :data="chartData" :options="chartOptions" />
  </div>
</template>

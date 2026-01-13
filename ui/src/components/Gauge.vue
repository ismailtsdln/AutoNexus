<script setup lang="ts">
import { computed } from "vue";

const props = defineProps({
  value: { type: Number, default: 0 },
  min: { type: Number, default: 0 },
  max: { type: Number, default: 100 },
  label: { type: String, default: "Value" },
  unit: { type: String, default: "" },
  color: { type: String, default: "#38bdf8" },
});

const percentage = computed(() => {
  const p = ((props.value - props.min) / (props.max - props.min)) * 100;
  return Math.min(Math.max(p, 0), 100);
});

const strokeDasharray = computed(() => {
  const radius = 40;
  const circumference = 2 * Math.PI * radius;
  const arcLength = (percentage.value / 100) * circumference;
  return `${arcLength} ${circumference}`;
});
</script>

<template>
  <div class="gauge-card">
    <div class="gauge-container">
      <svg viewBox="0 0 100 100" class="gauge-svg">
        <!-- Background Circle -->
        <circle
          cx="50"
          cy="50"
          r="40"
          fill="none"
          stroke="rgba(255,255,255,0.05)"
          stroke-width="8"
        />
        <!-- Progress Circle -->
        <circle
          cx="50"
          cy="50"
          r="40"
          fill="none"
          :stroke="color"
          stroke-width="8"
          stroke-linecap="round"
          :stroke-dasharray="strokeDasharray"
          transform="rotate(-90 50 50)"
          class="progress-circle"
        />
        <!-- Text -->
        <text x="50" y="45" text-anchor="middle" class="v-val">
          {{ value.toFixed(1) }}
        </text>
        <text x="50" y="65" text-anchor="middle" class="v-unit">
          {{ unit }}
        </text>
      </svg>
    </div>
    <div class="gauge-label">{{ label }}</div>
  </div>
</template>

<style scoped>
.gauge-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.05);
}

.gauge-container {
  width: 120px;
  height: 120px;
}

.gauge-svg {
  width: 100%;
  height: 100%;
}

.progress-circle {
  transition: stroke-dasharray 0.5s ease;
}

.v-val {
  fill: white;
  font-weight: bold;
  font-size: 16px;
}

.v-unit {
  fill: var(--text-secondary);
  font-size: 10px;
}

.gauge-label {
  margin-top: 8px;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-primary);
}
</style>

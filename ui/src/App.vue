<script setup lang="ts">
import { ref, onMounted } from "vue";
import SignalChart from "./components/SignalChart.vue";
import Gauge from "./components/Gauge.vue";
import ScriptEditor from "./components/ScriptEditor.vue";

const activeNav = ref("dashboard");
const rpm = ref(0);
const speed = ref(0);
const temp = ref(90);

onMounted(() => {
  setInterval(() => {
    rpm.value = 2000 + Math.random() * 500;
    speed.value = 80 + Math.random() * 5;
    temp.value = 90 + Math.random() * 2;
  }, 1000);
});
</script>

<template>
  <div class="titlebar">AutoNexus Core v1.0.0</div>

  <div class="dashboard">
    <aside class="sidebar">
      <div
        style="
          padding: 0 24px 24px 24px;
          display: flex;
          align-items: center;
          gap: 12px;
          margin-bottom: 8px;
        "
      >
        <img src="/assets/logo.svg" width="36" height="36" alt="Logo" />
        <span
          style="
            font-weight: 700;
            font-size: 16px;
            color: var(--text-primary);
            letter-spacing: -0.5px;
          "
          >AutoNexus</span
        >
      </div>
      <div
        class="nav-item"
        :class="{ active: activeNav === 'dashboard' }"
        @click="activeNav = 'dashboard'"
      >
        <span>📊</span> Dashboard
      </div>
      <div
        class="nav-item"
        :class="{ active: activeNav === 'traffic' }"
        @click="activeNav = 'traffic'"
      >
        <span>📡</span> Traffic Monitor
      </div>
      <div
        class="nav-item"
        :class="{ active: activeNav === 'diagnostics' }"
        @click="activeNav = 'diagnostics'"
      >
        <span>🩺</span> Diagnostics (UDS)
      </div>
      <div
        class="nav-item"
        :class="{ active: activeNav === 'scripts' }"
        @click="activeNav = 'scripts'"
      >
        <span>📜</span> Script Engine
      </div>
      <div
        class="nav-item"
        :class="{ active: activeNav === 'twin' }"
        @click="activeNav = 'twin'"
      >
        <span>🏎️</span> Digital Twin
      </div>
      <div
        class="nav-item"
        :class="{ active: activeNav === 'settings' }"
        @click="activeNav = 'settings'"
      >
        <span>⚙️</span> Settings
      </div>
    </aside>

    <main class="content">
      <div v-if="activeNav === 'dashboard'">
        <header style="margin-bottom: 32px">
          <h1>Welcome to <span class="text-accent">AutoNexus</span></h1>
          <p style="color: var(--text-secondary)">
            Next-generation automotive communication hub.
          </p>
        </header>

        <div class="stats-grid">
          <div class="card">
            <h3
              style="
                color: var(--text-secondary);
                font-size: 11px;
                text-transform: uppercase;
              "
            >
              Adapter Status
            </h3>
            <div style="font-size: 20px; font-weight: 600; margin-top: 4px">
              CONNECTED
            </div>
          </div>
          <div class="card">
            <h3
              style="
                color: var(--text-secondary);
                font-size: 11px;
                text-transform: uppercase;
              "
            >
              Traffic Rate
            </h3>
            <div style="font-size: 20px; font-weight: 600; margin-top: 4px">
              124 msgs/s
            </div>
          </div>
          <div class="card">
            <h3
              style="
                color: var(--text-secondary);
                font-size: 11px;
                text-transform: uppercase;
              "
            >
              Nodes
            </h3>
            <div style="font-size: 20px; font-weight: 600; margin-top: 4px">
              3 Active
            </div>
          </div>
        </div>

        <div
          class="gauge-grid"
          style="
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 16px;
            margin-bottom: 24px;
          "
        >
          <Gauge
            :value="rpm"
            :max="8000"
            label="Engine RPM"
            unit="RPM"
            color="#38bdf8"
          />
          <Gauge
            :value="speed"
            :max="220"
            label="Vehicle Speed"
            unit="km/h"
            color="#10b981"
          />
          <Gauge
            :value="temp"
            :max="150"
            label="Coolant Temp"
            unit="°C"
            color="#f59e0b"
          />
        </div>

        <div class="card" style="min-height: 410px">
          <div
            style="
              display: flex;
              justify-content: space-between;
              align-items: center;
              margin-bottom: 24px;
            "
          >
            <h2>Live Signal Monitor</h2>
            <div style="display: flex; gap: 8px">
              <span
                class="text-accent"
                style="font-size: 12px; font-weight: 600"
                >● LIVE</span
              >
            </div>
          </div>
          <SignalChart />
        </div>
      </div>

      <div v-if="activeNav === 'scripts'">
        <header style="margin-bottom: 32px">
          <h1>Cloud <span class="text-accent">Automation</span></h1>
          <p style="color: var(--text-secondary)">
            Script your automotive tests with high-level TypeScript API.
          </p>
        </header>
        <ScriptEditor />
      </div>
    </main>
  </div>
</template>

<style>
@import "./style.css";
</style>

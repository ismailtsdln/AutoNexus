<script setup lang="ts">
import { ref } from "vue";
import SignalChart from "./components/SignalChart.vue";

const activeNav = ref("dashboard");
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
                font-size: 13px;
                text-transform: uppercase;
              "
            >
              Adapter Status
            </h3>
            <div style="font-size: 24px; font-weight: 600; margin-top: 8px">
              CONNECTED
            </div>
            <div
              style="
                color: var(--accent-blue);
                font-size: 12px;
                margin-top: 4px;
              "
            >
              MockAdapter (Virtual)
            </div>
          </div>
          <div class="card">
            <h3
              style="
                color: var(--text-secondary);
                font-size: 13px;
                text-transform: uppercase;
              "
            >
              Traffic Rate
            </h3>
            <div style="font-size: 24px; font-weight: 600; margin-top: 8px">
              124 <span style="font-size: 14px; font-weight: 400">msgs/s</span>
            </div>
            <div style="color: #10b981; font-size: 12px; margin-top: 4px">
              ↑ Stabilized
            </div>
          </div>
          <div class="card">
            <h3
              style="
                color: var(--text-secondary);
                font-size: 13px;
                text-transform: uppercase;
              "
            >
              Active Protocols
            </h3>
            <div style="font-size: 24px; font-weight: 600; margin-top: 8px">
              CAN-FD, UDS
            </div>
            <div
              style="
                color: var(--text-secondary);
                font-size: 12px;
                margin-top: 4px;
              "
            >
              3 nodes discovered
            </div>
          </div>
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
    </main>
  </div>
</template>

<style>
@import "./style.css";
</style>

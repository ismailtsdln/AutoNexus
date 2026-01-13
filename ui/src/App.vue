<script setup lang="ts">
import { ref } from "vue";

const activeNav = ref("dashboard");
</script>

<template>
  <div class="titlebar">AutoNexus Core v1.0.0</div>

  <div class="dashboard">
    <aside class="sidebar">
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

        <div class="card" style="min-height: 200px">
          <h2>Recent Activity</h2>
          <div
            style="
              margin-top: 16px;
              display: flex;
              flex-direction: column;
              gap: 12px;
            "
          >
            <div
              style="
                border-left: 2px solid var(--accent-blue);
                padding-left: 12px;
              "
            >
              <div style="font-size: 14px">
                UDS: DiagnosticSessionControl(0x03)
              </div>
              <div style="font-size: 12px; color: var(--text-secondary)">
                Response: [11 22 33] • 2m ago
              </div>
            </div>
            <div
              style="
                border-left: 2px solid var(--accent-silver);
                padding-left: 12px;
              "
            >
              <div style="font-size: 14px">
                CAN: Frame 0x123 sent successfully
              </div>
              <div style="font-size: 12px; color: var(--text-secondary)">
                Length: 8 bytes • 5m ago
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<style>
@import "./style.css";
</style>

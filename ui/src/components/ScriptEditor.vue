<script setup lang="ts">
import { ref } from "vue";

const code = ref(`// AutoNexus Automation Script
const uds = new AutoNexus.UdsSession(new AutoNexus.MockCanAdapter());

async function run() {
    console.log("Starting diagnostic session...");
    await uds.diagnosticSessionControl(0x03);
    console.log("Reading Device ID...");
    const data = await uds.readDataByIdentifier(0xF190);
    console.log("Device ID Data:", data);
}

run();`);

const logs = ref<string[]>([]);
const isRunning = ref(false);

const runScript = () => {
  isRunning.value = true;
  logs.value.push("[SYSTEM] Executing script...");
  // Future: Connect to real SDK runner
  setTimeout(() => {
    logs.value.push("[LOG] Starting diagnostic session...");
    logs.value.push(
      "[LOG] Device ID Data: [0x56, 0x49, 0x4E, 0x31, 0x32, 0x33]"
    );
    logs.value.push("[SYSTEM] Script finished successfully.");
    isRunning.value = false;
  }, 1500);
};

const clearLogs = () => {
  logs.value = [];
};
</script>

<template>
  <div class="script-editor-container card">
    <div class="card-header">
      <h3 class="card-title">📜 Automation IDE</h3>
      <div class="actions">
        <button
          @click="runScript"
          :disabled="isRunning"
          class="btn btn-primary"
        >
          {{ isRunning ? "Running..." : "▶ Run Script" }}
        </button>
        <button @click="clearLogs" class="btn btn-secondary">Clear Logs</button>
      </div>
    </div>
    <div class="editor-layout">
      <textarea v-model="code" class="code-area" spellcheck="false"></textarea>
      <div class="log-area">
        <div v-for="(log, i) in logs" :key="i" class="log-line">
          {{ log }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.script-editor-container {
  display: flex;
  flex-direction: column;
  height: 500px;
}

.editor-layout {
  display: flex;
  flex: 1;
  gap: 12px;
  overflow: hidden;
  padding: 16px;
}

.code-area {
  flex: 2;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: #a5d6ff;
  font-family: "Fira Code", monospace;
  font-size: 13px;
  padding: 12px;
  resize: none;
  outline: none;
}

.log-area {
  flex: 1;
  background: #0d1117;
  border-radius: 8px;
  padding: 12px;
  font-family: monospace;
  font-size: 12px;
  overflow-y: auto;
  color: #8b949e;
}

.log-line {
  margin-bottom: 4px;
  border-bottom: 1px solid #21262d;
  padding-bottom: 2px;
}

.btn {
  padding: 6px 14px;
  border-radius: 6px;
  border: none;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.2s;
}

.btn-primary {
  background: var(--accent-blue);
  color: white;
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.1);
  color: var(--text-secondary);
}

.btn:hover:not(:disabled) {
  opacity: 0.8;
  transform: translateY(-1px);
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.actions {
  display: flex;
  gap: 8px;
}
</style>

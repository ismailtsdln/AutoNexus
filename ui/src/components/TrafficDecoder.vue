<script setup lang="ts">
import { ref, onMounted } from "vue";

interface DecodedSignal {
  name: string;
  value: number;
  unit: string;
}

interface CanMessage {
  id: string;
  name: string;
  data: string;
  timestamp: string;
  signals: DecodedSignal[];
  isExpanded: boolean;
}

const messages = ref<CanMessage[]>([]);
const filter = ref("");

const addSampleMessage = () => {
  const msg: CanMessage = {
    id: "0x123",
    name: "EngineData",
    data: "11 22 33 44 55 66 77 88",
    timestamp: new Date().toLocaleTimeString(),
    isExpanded: false,
    signals: [
      { name: "EngineRPM", value: 2500 + Math.random() * 100, unit: "RPM" },
      { name: "CoolantTemp", value: 90 + Math.random() * 2, unit: "°C" },
    ],
  };
  messages.value.unshift(msg);
  if (messages.value.length > 50) messages.value.pop();
};

onMounted(() => {
  setInterval(addSampleMessage, 1000);
});

const toggleExpand = (msg: CanMessage) => {
  msg.isExpanded = !msg.isExpanded;
};
</script>

<template>
  <div class="traffic-decoder card">
    <div class="card-header">
      <h3 class="card-title">📡 Real-time Traffic Decoder</h3>
      <input
        v-model="filter"
        placeholder="Filter by ID or Name..."
        class="filter-input"
      />
    </div>
    <div class="table-container">
      <table class="traffic-table">
        <thead>
          <tr>
            <th>Time</th>
            <th>ID</th>
            <th>Name</th>
            <th>Data (Hex)</th>
            <th>Action</th>
          </tr>
        </thead>
        <tbody>
          <template v-for="(msg, i) in messages" :key="i">
            <tr :class="{ 'row-expanded': msg.isExpanded }">
              <td>{{ msg.timestamp }}</td>
              <td class="text-accent">{{ msg.id }}</td>
              <td>
                <strong>{{ msg.name }}</strong>
              </td>
              <td class="hex-data">{{ msg.data }}</td>
              <td>
                <button @click="toggleExpand(msg)" class="btn-tiny">
                  {{ msg.isExpanded ? "Collapse" : "Signals" }}
                </button>
              </td>
            </tr>
            <tr v-if="msg.isExpanded" class="signal-row">
              <td colspan="5">
                <div class="signal-grid">
                  <div
                    v-for="sig in msg.signals"
                    :key="sig.name"
                    class="signal-item"
                  >
                    <span class="sig-name">{{ sig.name }}:</span>
                    <span class="sig-val">{{ sig.value.toFixed(2) }}</span>
                    <span class="sig-unit">{{ sig.unit }}</span>
                  </div>
                </div>
              </td>
            </tr>
          </template>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
.traffic-decoder {
  display: flex;
  flex-direction: column;
  height: 600px;
}

.filter-input {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: white;
  padding: 6px 12px;
  font-size: 13px;
  outline: none;
}

.table-container {
  flex: 1;
  overflow-y: auto;
}

.traffic-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.traffic-table th {
  text-align: left;
  padding: 12px;
  background: rgba(0, 0, 0, 0.2);
  color: var(--text-secondary);
  text-transform: uppercase;
  font-size: 11px;
}

.traffic-table td {
  padding: 10px 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.03);
}

.hex-data {
  font-family: monospace;
  color: #a5d6ff;
}

.btn-tiny {
  background: var(--accent-blue);
  color: white;
  border: none;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 11px;
  cursor: pointer;
}

.row-expanded {
  background: rgba(56, 189, 248, 0.05);
}

.signal-row {
  background: rgba(0, 0, 0, 0.2);
}

.signal-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 12px;
  padding: 12px 24px;
}

.signal-item {
  display: flex;
  gap: 6px;
  align-items: center;
}

.sig-name {
  color: var(--text-secondary);
  font-size: 12px;
}

.sig-val {
  color: var(--accent-blue);
  font-weight: bold;
}

.sig-unit {
  font-size: 10px;
  color: rgba(255, 255, 255, 0.4);
}
</style>

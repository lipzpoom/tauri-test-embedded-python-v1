<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");
const tradingHistory = ref([]);
const loading = ref(false);
const error = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function fetchTradingHistory() {
  loading.value = true;
  error.value = "";
  try {
    const result = await invoke("get_trading_history");
    if (result.success) {
      tradingHistory.value = result.data;
    } else {
      error.value = result.error || "Failed to fetch trading history";
    }
  } catch (err) {
    error.value = "Error fetching trading history: " + err.message;
  } finally {
    loading.value = false;
  }
}

// Add helper function for time formatting
function formatTime(timestamp) {
  return new Date(timestamp).toLocaleString();
}

// Load trading history on component mount
onMounted(() => {
  fetchTradingHistory();
});
</script>

<template>
  <main class="container">
    <h1>Trading History Dashboard</h1>

    <div class="row">
      <a href="https://vite.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>

    <!-- Trading History Section -->
    <section class="trading-section">
      <div class="section-header">
        <h2>Trading History</h2>
        <button
          @click="fetchTradingHistory"
          :disabled="loading"
          class="refresh-btn"
        >
          {{ loading ? "Loading..." : "Refresh" }}
        </button>
      </div>

      <div v-if="error" class="error-message">
        {{ error }}
      </div>

      <div v-if="loading" class="loading">Loading trading history...</div>

      <div v-else-if="tradingHistory.length > 0" class="trading-table">
        <table>
          <thead>
            <tr>
              <th>Symbol</th>
              <th>Side</th>
              <th>Quantity</th>
              <th>Price</th>
              <th>Status</th>
              <th>Time</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="trade in tradingHistory" :key="trade.id">
              <td>{{ trade.symbol }}</td>
              <td :class="trade.side.toLowerCase()">{{ trade.side }}</td>
              <td>{{ trade.quantity }}</td>
              <td>${{ trade.price }}</td>
              <td :class="trade.status.toLowerCase()">{{ trade.status }}</td>
              <td>{{ formatTime(trade.timestamp) }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div v-else class="no-data">No trading history available</div>
    </section>

    <!-- Original Greet Section -->
    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.trading-section {
  margin: 2rem 0;
  padding: 1rem;
  border-radius: 8px;
  background-color: rgba(255, 255, 255, 0.1);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.section-header h2 {
  margin: 0;
  color: #646cff;
}

.refresh-btn {
  background-color: #646cff;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
}

.refresh-btn:hover {
  background-color: #535bf2;
}

.refresh-btn:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.trading-table {
  overflow-x: auto;
}

.trading-table table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 1rem;
}

.trading-table th,
.trading-table td {
  text-align: left;
  padding: 0.75rem;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}

.trading-table th {
  background-color: rgba(100, 108, 255, 0.1);
  font-weight: 600;
  color: #646cff;
}

.trading-table tbody tr:hover {
  background-color: rgba(100, 108, 255, 0.05);
}

.buy {
  color: #22c55e;
  font-weight: 600;
}

.sell {
  color: #ef4444;
  font-weight: 600;
}

.filled {
  color: #22c55e;
  font-weight: 500;
}

.pending {
  color: #f59e0b;
  font-weight: 500;
}

.error-message {
  color: #ef4444;
  background-color: rgba(239, 68, 68, 0.1);
  padding: 1rem;
  border-radius: 4px;
  margin: 1rem 0;
}

.loading,
.no-data {
  text-align: center;
  padding: 2rem;
  color: #666;
  font-style: italic;
}

@media (prefers-color-scheme: dark) {
  .trading-section {
    background-color: rgba(255, 255, 255, 0.05);
  }

  .trading-table th {
    background-color: rgba(36, 200, 219, 0.1);
    color: #24c8db;
  }

  .section-header h2 {
    color: #24c8db;
  }

  .refresh-btn {
    background-color: #24c8db;
  }

  .refresh-btn:hover {
    background-color: #22b5c7;
  }

  .trading-table th,
  .trading-table td {
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .trading-table tbody tr:hover {
    background-color: rgba(36, 200, 219, 0.1);
  }
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>

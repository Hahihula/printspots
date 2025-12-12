<template>
  <div class="completion-screen">
    <div class="content-wrapper">


      <!-- Title and Message -->
      <h1 class="title">3MF Generated Successfully!</h1>
      <p class="subtitle">Your printspot is ready to print</p>

      <!-- Success Icon -->
      <div class="success-icon">
        <svg xmlns="http://www.w3.org/2000/svg" class="checkmark" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
        </svg>
      </div>

      <!-- Stats Grid -->
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-label">Black Mesh</div>
          <div class="stat-row">
            <span>Vertices:</span>
            <span class="stat-value">{{ meshStats.black_vertices.toLocaleString() }}</span>
          </div>
          <div class="stat-row">
            <span>Triangles:</span>
            <span class="stat-value">{{ meshStats.black_triangles.toLocaleString() }}</span>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-label">White Mesh</div>
          <div class="stat-row">
            <span>Vertices:</span>
            <span class="stat-value">{{ meshStats.white_vertices.toLocaleString() }}</span>
          </div>
          <div class="stat-row">
            <span>Triangles:</span>
            <span class="stat-value">{{ meshStats.white_triangles.toLocaleString() }}</span>
          </div>
        </div>
      </div>

      <!-- File Path -->
      <div class="file-info">
        <div class="file-path-label">Saved to:</div>
        <div class="file-path">{{ meshStats.output_path }}</div>
      </div>

      <!-- Important Notice -->
      <div class="notice">
        <svg xmlns="http://www.w3.org/2000/svg" class="notice-icon" fill="none" viewBox="0 0 24 24"
          stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
        </svg>
        <div>
          <div class="notice-title">Important Slicer Settings</div>
          <ul class="notice-list">
            <li>Layer Height: <strong>{{ layerThickness }}mm</strong></li>
            <li>Infill: <strong>100%</strong> for both meshes</li>
          </ul>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="actions">
        <button @click="openFolder" class="btn-secondary">
          <svg xmlns="http://www.w3.org/2000/svg" class="btn-icon" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
          Open Folder
        </button>
        <button @click="startNewProject" class="btn-primary">
          <svg xmlns="http://www.w3.org/2000/svg" class="btn-icon" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          New Project
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  meshStats: {
    type: Object,
    required: true
  },
  layerThickness: {
    type: Number,
    required: true
  }
});

const emit = defineEmits(['start-new-project']);

async function openFolder() {
  try {
    await invoke('show_in_folder', { path: props.meshStats.output_path });
  } catch (e) {
    console.error('Failed to open folder:', e);
  }
}

function startNewProject() {
  emit('start-new-project');
}
</script>

<style scoped>
.completion-screen {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  background: linear-gradient(135deg, rgba(0, 188, 212, 0.05) 0%, rgba(233, 30, 99, 0.05) 100%);
  padding: 2rem;
  overflow-y: auto;
}

.content-wrapper {
  max-width: 700px;
  width: 100%;
  text-align: center;
}

.success-icon {
  width: 80px;
  height: 80px;
  margin: 0 auto 2rem;
  background: linear-gradient(135deg, var(--primary), var(--secondary));
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: scaleIn 0.5s ease-out;
}

.checkmark {
  width: 48px;
  height: 48px;
  color: white;
  stroke-width: 3;
}

@keyframes scaleIn {
  from {
    transform: scale(0);
    opacity: 0;
  }

  to {
    transform: scale(1);
    opacity: 1;
  }
}

.title {
  font-size: 2rem;
  font-weight: 800;
  color: white;
  margin-bottom: 0.5rem;
}

.subtitle {
  font-size: 1.125rem;
  color: var(--text-muted);
  margin-bottom: 3rem;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
  margin-bottom: 2rem;
}

.stat-card {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 1.5rem;
  transition: all 0.3s;
}

.stat-card:hover {
  background: rgba(255, 255, 255, 0.08);
  transform: translateY(-2px);
}

.stat-label {
  font-size: 0.875rem;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 1rem;
  font-weight: 700;
  border-bottom: 2px solid rgba(255, 255, 255, 0.1);
  padding-bottom: 0.5rem;
}

.stat-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.875rem;
  color: var(--text);
  margin-bottom: 0.5rem;
}

.stat-row:last-child {
  margin-bottom: 0;
}

.stat-value {
  font-weight: 700;
  color: var(--primary);
  font-family: 'Courier New', monospace;
}

.file-info {
  background: rgba(0, 0, 0, 0.2);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  padding: 1rem;
  margin-bottom: 2rem;
}

.file-path-label {
  font-size: 0.75rem;
  color: var(--text-muted);
  margin-bottom: 0.5rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-weight: 600;
}

.file-path {
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
  color: var(--text);
  word-break: break-all;
}

.notice {
  background: rgba(255, 165, 0, 0.1);
  border: 1px solid rgba(255, 165, 0, 0.3);
  border-radius: 8px;
  padding: 1rem;
  margin-bottom: 2rem;
  display: flex;
  gap: 1rem;
  text-align: left;
}

.notice-icon {
  width: 24px;
  height: 24px;
  color: orange;
  flex-shrink: 0;
}

.notice-title {
  font-weight: 700;
  color: orange;
  margin-bottom: 0.5rem;
}

.notice-list {
  list-style: none;
  padding: 0;
  margin: 0;
  font-size: 0.875rem;
  color: var(--text);
}

.notice-list li {
  margin-bottom: 0.25rem;
}

.actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.btn-primary,
.btn-secondary {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.875rem 1.75rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn-primary {
  background: linear-gradient(135deg, var(--primary), var(--secondary));
  color: white;
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 188, 212, 0.3);
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: white;
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.1);
  transform: translateY(-2px);
}

.btn-icon {
  width: 20px;
  height: 20px;
}
</style>

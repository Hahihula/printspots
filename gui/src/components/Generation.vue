<template>
  <div class="workspace">
    <div class="toolbar">
      <div class="tool-group">
        <label>Printer:</label>
        <select class="tool-select">
          <option>MyLovelyPrinter 0.4 Nozzle</option>
          <option>Ender 3 V2</option>
        </select>
      </div>
      <div class="tool-group">
        <label>Palette:</label>
        <select class="tool-select">
          <option>Grayscale Standard</option>
          <option>Sepia Tone</option>
        </select>
      </div>
      <div class="spacer"></div>
      <button class="btn btn-secondary" @click="openImage">📂 Open Image</button>
    </div>

    <div class="workspace-body">
      <div class="sidebar">
        <div class="control-section">
          <h4>Image Adjustments</h4>
          
          <div class="form-group">
            <label class="form-label">Contrast</label>
            <input type="range" class="range-sm" v-model="params.contrast">
          </div>
          
          <div class="form-group">
            <label class="form-label">Brightness</label>
            <input type="range" class="range-sm" v-model="params.brightness">
          </div>

          <div class="form-group">
            <label class="form-label">Smoothing</label>
             <input type="range" class="range-sm" v-model="params.smoothing">
          </div>
        </div>

        <div class="control-section">
          <h4>Dimensions</h4>
          <div class="form-group">
             <label class="form-label">Width (mm)</label>
             <input type="number" class="form-input input-sm" v-model="params.width">
          </div>
        </div>

        <div class="sidebar-footer">
            <button class="btn btn-primary btn-cmyk full-btn" @click="generateModel">
              GENERATE 3D MODEL
            </button>
        </div>
      </div>

      <div class="viewport">
        <div class="view-toggles">
          <button :class="{active: viewMode==='2d'}" @click="viewMode='2d'">2D Preview</button>
          <button :class="{active: viewMode==='3d'}" @click="viewMode='3d'" :disabled="!hasGenerated">3D Result</button>
        </div>

        <div class="view-content">
          <div v-if="viewMode==='2d'" class="image-preview">
             <img src="https://via.placeholder.com/800x600?text=Load+Image" class="preview-img" />
             </div>

          <div v-if="viewMode==='3d'" class="three-container">
            <p>3D Viewer Component Here</p>
          </div>
        </div>
      </div>
    </div>

    <div class="status-bar">
      <span class="status-text">Ready</span>
      <div class="progress-track" v-if="isGenerating">
        <div class="progress-fill"></div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const viewMode = ref('2d');
const hasGenerated = ref(false);
const isGenerating = ref(false);

const params = ref({
  contrast: 50,
  brightness: 50,
  smoothing: 10,
  width: 150
});

const openImage = () => {
    // Tauri dialog open
    console.log("Open image dialog");
};

const generateModel = () => {
    isGenerating.value = true;
    setTimeout(() => {
        isGenerating.value = false;
        hasGenerated.value = true;
        viewMode.value = '3d';
    }, 2000); // Simulate Rust work
};
</script>

<style scoped>
.workspace {
  display: flex;
  flex-direction: column;
  height: 100%; /* KEEP THIS */ 
  width: 100%;
}
/* This must take up all the remaining space and be a flex container */
.workspace-body {
  flex: 1; /* This tells it to use the remaining height */
  display: flex;
  overflow: hidden;
}

/* Ensure the central viewport fills its height for the 3D viewer */
.viewport {
  flex: 1;
  background: #0a0a0a;
  position: relative;
  display: flex;
  flex-direction: column; /* Added this to manage the view-toggles and view-content */
}
.view-content {
  flex: 1; /* This ensures the content area takes the space */
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}
.toolbar {
  height: 50px;
  background: var(--dark-lighter);
  border-bottom: 1px solid rgba(255,255,255,0.05);
  display: flex;
  align-items: center;
  padding: 0 1rem;
  gap: 1rem;
}
.tool-group {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--text-muted);
  font-size: 0.9rem;
}
.tool-select {
  background: rgba(0,0,0,0.3);
  border: 1px solid rgba(255,255,255,0.1);
  color: var(--text);
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
}
.spacer { flex: 1; }

.workspace-body {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.sidebar {
  width: 280px;
  background: var(--dark-lighter);
  border-right: 1px solid rgba(255,255,255,0.05);
  display: flex;
  flex-direction: column;
  padding: 1rem;
}
.control-section {
  margin-bottom: 2rem;
  border-bottom: 1px solid rgba(255,255,255,0.05);
  padding-bottom: 1rem;
}
.control-section h4 {
  color: var(--primary);
  margin-bottom: 1rem;
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 1px;
}
.sidebar-footer {
  margin-top: auto;
}
.full-btn { width: 100%; padding: 1rem; }

.viewport {
  flex: 1;
  background: #0a0a0a;
  position: relative;
  display: flex;
  flex-direction: column;
}
.view-toggles {
  position: absolute;
  top: 1rem;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(0,0,0,0.5);
  backdrop-filter: blur(5px);
  padding: 0.25rem;
  border-radius: 20px;
  display: flex;
  gap: 0.5rem;
  z-index: 10;
}
.view-toggles button {
  background: transparent;
  border: none;
  color: var(--text-muted);
  padding: 0.5rem 1rem;
  border-radius: 15px;
  cursor: pointer;
  transition: 0.2s;
}
.view-toggles button.active {
  background: var(--primary);
  color: white;
}
.view-content {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}
.preview-img {
  max-width: 90%;
  max-height: 90%;
  box-shadow: 0 0 50px rgba(0,0,0,0.5);
}

.status-bar {
  height: 30px;
  background: #000;
  display: flex;
  align-items: center;
  padding: 0 1rem;
  font-size: 0.8rem;
  color: var(--text-muted);
  position: relative;
}
.progress-track {
  position: absolute;
  top: 0; left: 0; width: 100%; height: 2px;
  background: rgba(255,255,255,0.1);
}
.progress-fill {
  height: 100%;
  width: 50%; /* Dynamic */
  background: linear-gradient(90deg, var(--primary), var(--secondary), var(--accent));
  animation: loading 2s infinite;
}
@keyframes loading {
  0% { transform: translateX(-100%); }
  100% { transform: translateX(200%); }
}
</style>
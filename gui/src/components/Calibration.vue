<template>
  <div class="view-container">
    <div class="calibration-header">
      <div class="tabs">
        <button :class="['tab-btn', { active: step === 1 }]" @click="step = 1">1. Generate Pattern</button>
        <button :class="['tab-btn', { active: step === 2 }]" @click="step = 2">2. Create Palette</button>
      </div>
    </div>

    <div v-if="step === 1" class="step-content">
      <div class="panel center-panel">
        <i class="feature-icon">🏗️</i>
        <h3>Print the Calibration Stick</h3>
        <p>This will generate a .3mf file with stepped layers. Print this with your base color (Black) and top color (White).</p>
        <button class="btn btn-primary" @click="generateCalibrationFile">
          Generate calibration.3mf
        </button>
      </div>
    </div>

    <div v-else class="step-content split-view">
      <div class="canvas-area">
        <div v-if="!calibrationImage" class="upload-zone" @click="triggerUpload">
          <p>Click to Upload Photo of Print</p>
        </div>
        <div v-else class="image-wrapper">
          <img :src="calibrationImage" alt="Calibration Scan" />
          <div class="overlay-instructions">Click the squares from Dark (1) to Light (Max)</div>
        </div>
      </div>

      <div class="palette-sidebar">
        <h3>New Palette</h3>
        <div class="form-group">
          <label class="form-label">Palette Name</label>
          <input type="text" class="form-input" placeholder="e.g. PLA Matte Gray">
        </div>

        <div class="color-list">
          <div class="list-header">Selected Colors</div>
          <div class="empty-state" v-if="colors.length === 0">No colors picked yet</div>
          </div>

        <button class="btn btn-primary btn-cmyk full-width">Save Palette</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const step = ref(1);
const calibrationImage = ref(null);
const colors = ref([]);

const generateCalibrationFile = () => {
  // Call Rust command
  alert("Generating 3mf...");
};

const triggerUpload = () => {
  // Mock upload
  calibrationImage.value = "https://via.placeholder.com/600x150"; 
};
</script>

<style scoped>
.view-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
}
.calibration-header {
  padding: 1rem 2rem;
  background: var(--dark-lighter);
  border-bottom: 1px solid rgba(255,255,255,0.05);
}
.tabs {
  display: flex;
  gap: 1rem;
}
.tab-btn {
  background: transparent;
  border: none;
  color: var(--text-muted);
  font-size: 1.1rem;
  padding: 0.5rem 1rem;
  cursor: pointer;
  border-bottom: 2px solid transparent;
}
.tab-btn.active {
  color: var(--primary);
  border-bottom-color: var(--primary);
}
.step-content {
  flex: 1;
  padding: 2rem;
  overflow: hidden;
}
.center-panel {
  max-width: 500px;
  margin: 4rem auto;
  text-align: center;
}
.feature-icon {
  font-size: 4rem;
  display: block;
  margin-bottom: 1rem;
}
.split-view {
  display: grid;
  grid-template-columns: 1fr 300px;
  gap: 2rem;
  height: 100%;
}
.canvas-area {
  background: #000;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  overflow: hidden;
}
.upload-zone {
  border: 2px dashed var(--text-muted);
  padding: 4rem;
  border-radius: 8px;
  cursor: pointer;
}
.upload-zone:hover {
  border-color: var(--primary);
  color: var(--primary);
}
.palette-sidebar {
  background: var(--dark-lighter);
  padding: 1.5rem;
  border-radius: 8px;
}
.full-width {
  width: 100%;
  margin-top: 1rem;
}
</style>
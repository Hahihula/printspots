<template>
  <div class="view-container">
    <div class="config-grid">
      <div class="panel">
        <h2 class="section-title" style="text-align: left; margin-bottom: 2rem;">Printer Setup</h2>
        
        <form @submit.prevent="saveConfig">
          <div class="form-group">
            <label class="form-label">Configuration Name</label>
            <input type="text" class="form-input" v-model="config.name" placeholder="e.g. MyLovelyPrinter 0.4 Nozzle">
          </div>

          <div class="form-row">
            <div class="form-group">
              <label class="form-label">Base Thickness (mm)</label>
              <input type="number" step="0.1" class="form-input" v-model.number="config.base_thickness">
              <small class="hint">Thickness of the solid backplate</small>
            </div>
            
            <div class="form-group">
              <label class="form-label">Layer Height (mm)</label>
              <input type="number" step="0.01" class="form-input" v-model.number="config.layer_height">
              <small class="hint">Must match your slicer setting. It's recomended to use the minimal layer height your printer is capable of.</small>
            </div>
          </div>

          <div class="form-group">
            <label class="form-label">Max Layers</label>
            <div class="slider-container">
              <input type="range" min="10" max="1000" v-model.number="config.max_layers">
              <span class="value-badge">{{ config.max_layers }}</span>
            </div>
          </div>

          <div class="actions">
            <button class="btn btn-primary btn-cmyk" type="submit">Save Configuration</button>
          </div>
        </form>
      </div>

      <div class="panel preview-panel">
        <h3>Calculated Dimensions</h3>
        <div class="dimension-box">
          <div class="dim-label">Total Height</div>
          <div class="dim-value">{{ totalHeight }} <span class="unit">mm</span></div>
        </div>
        <div class="dimension-visualizer">
          <div class="vis-base" :style="{ height: (config.base_thickness * 10) + 'px' }"></div>
          <div class="vis-layers" :style="{ height: (config.layer_height * config.max_layers * 10) + 'px' }"></div>
        </div>
        <p class="text-muted">
          This configuration ensures your printspots fits within your printer's Z-axis capabilities.
        </p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed, ref } from 'vue';

const config = ref({
  name: 'My Printer',
  base_thickness: 0.6,
  layer_height: 0.08,
  max_layers: 150
});

const totalHeight = computed(() => {
  return (config.value.base_thickness + (config.value.layer_height * config.value.max_layers)).toFixed(2);
});

const saveConfig = () => {
  // TODO: Call Rust Command to save config
  console.log("Saving config:", config.value);
};
</script>

<style scoped>
.view-container {
  padding: 2rem;
  width: 100%;
  overflow-y: auto;
}
.config-grid {
  display: grid;
  grid-template-columns: 1fr 300px;
  gap: 2rem;
  max-width: 1000px;
  margin: 0 auto;
}
.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}
.hint {
  color: var(--text-muted);
  font-size: 0.75rem;
  margin-top: 0.25rem;
  display: block;
}
.value-badge {
  background: var(--dark);
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  min-width: 40px;
  text-align: center;
}
.dimension-box {
  background: rgba(0,0,0,0.3);
  padding: 1rem;
  border-radius: 8px;
  text-align: center;
  margin: 1rem 0;
}
.dim-value {
  font-size: 2rem;
  color: var(--primary);
  font-weight: bold;
}
.dimension-visualizer {
  display: flex;
  flex-direction: column-reverse; /* Build from bottom up */
  align-items: center;
  justify-content: flex-start;
  height: 200px;
  background: rgba(255,255,255,0.02);
  border-bottom: 2px solid var(--text-muted);
  margin-top: 2rem;
  padding-bottom: 1px;
}
.vis-base {
  width: 60px;
  background: var(--text-muted);
  transition: height 0.3s;
}
.vis-layers {
  width: 60px;
  background: linear-gradient(to top, var(--primary-dark), var(--primary));
  transition: height 0.3s;
  opacity: 0.8;
}
</style>
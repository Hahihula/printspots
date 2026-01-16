<template>
  <div class="image-container" :class="{ 'eyedropper-active': isEyedropperActive }" @click="$emit('pick-color-with-eyedropper', $event)">
    <img :src="transformedImageUrl || imageUrl"
         ref="calibrationImage"
         class="calibration-image"
         @load="$emit('image-load', $refs.calibrationImage)">
    <div class="grid-overlay" v-if="gridReady && !hideGrid">
      <div v-for="(cell, index) in gridCells"
           :key="index"
           class="grid-cell"
           :class="{ picked: colors[cell.layerIndex] }"
           :style="getCellStyle(cell)"
           @click.stop="!isEyedropperActive && $emit('pick-color', cell)">
        <span class="cell-label">{{ cell.layerIndex }}</span>
      </div>
    </div>
  </div>

  <div class="grid-adjust">
    <div class="grid-adjust-row">
      <label>Fine-tune grid position:</label>
    </div>
    <div class="grid-adjust-row">
      <label>X offset:</label>
      <input type="number" :value="gridOffset.x" @input="handleGridOffsetInput('x', $event)" class="small-input">
      <label>Y offset:</label>
      <input type="number" :value="gridOffset.y" @input="handleGridOffsetInput('y', $event)" class="small-input">
    </div>
    <div class="grid-adjust-row">
      <label>Scale:</label>
      <input type="number" :value="gridScale" @input="handleGridScaleInput" class="small-input" step="0.01" min="0.5" max="1.5">
      <button @click="$emit('auto-detect-grid')" class="btn btn-secondary" style="margin-left: auto;">Auto-Fit</button>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Grid',
  emits: ['image-load', 'pick-color', 'update-grid-offset', 'update-grid-scale', 'auto-detect-grid', 'pick-color-with-eyedropper'],
  props: {
    imageUrl: String,
    transformedImageUrl: String,
    gridReady: Boolean,
    gridCells: Array,
    colors: Object,
    gridOffset: Object,
    gridScale: Number,
    isEyedropperActive: Boolean,
    hideGrid: Boolean
  },
  methods: {
    getCellStyle(cell) {
      return {
        left: cell.x + 'px',
        top: cell.y + 'px',
        width: cell.width + 'px',
        height: cell.height + 'px'
      };
    },
    handleGridOffsetInput(axis, event) {
      const value = event.target.value;
      if (value === '') {
        this.$emit('update-grid-offset', { axis, value: 0 });
        return;
      }
      const parsedValue = parseFloat(value);
      if (!isNaN(parsedValue)) {
        this.$emit('update-grid-offset', { axis, value: parsedValue });
      }
    },
    handleGridScaleInput(event) {
      const value = event.target.value;
      if (value === '') {
        this.$emit('update-grid-scale', 0);
        return;
      }
      const parsedValue = parseFloat(value);
      if (!isNaN(parsedValue)) {
        this.$emit('update-grid-scale', parsedValue);
      }
    }
  }
}
</script>

<style scoped>
.image-container {
  position: relative;
  display: inline-block;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  overflow: hidden;
}

.image-container.eyedropper-active {
  cursor: crosshair;
}

.calibration-image {
  display: block;
  max-width: 100%;
  height: auto;
}

.grid-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}

.grid-cell {
  position: absolute;
  border: 1px solid rgba(0, 0, 255, 0.3);
  cursor: pointer;
  pointer-events: all;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.grid-cell:hover {
  border: 2px solid #2196F3;
  background: rgba(33, 150, 243, 0.1);
}

.grid-cell.picked {
  border: 2px solid #4CAF50;
  background: rgba(76, 175, 80, 0.1);
}

.cell-label {
  background: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: bold;
}

.grid-adjust {
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  padding: 10px;
  margin-top: 10px;
  color: #1a1a1a; /* Dark text for readability on white background */
}

.grid-adjust-row {
  display: flex;
  gap: 10px;
  align-items: center;
  margin-bottom: 10px;
}

.grid-adjust-row:last-child {
  margin-bottom: 0;
}

.small-input {
  width: 60px;
  padding: 5px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary {
  background: #2196F3;
  color: white;
  margin-right: 10px;
}

.btn-secondary:hover {
  background: #1976D2;
}
</style>

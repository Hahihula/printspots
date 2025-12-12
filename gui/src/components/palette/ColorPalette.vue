<template>
  <div class="color-palette">
    <h3>Extracted Colors</h3>
    <div class="palette-grid">
      <div v-for="i in maxLayers + 1"
           :key="i"
           class="color-box"
           :class="{ active: selectedLayer === i - 1 }"
           :style="{ backgroundColor: getColorDisplay(i - 1) }"
           @click="$emit('update:selectedLayer', i - 1)">
        <span class="color-index">{{ i - 1 }}</span>
      </div>
    </div>

    <div class="status">
      {{ pickedCount }} / {{ maxLayers + 1 }} colors picked
    </div>

    <div class="actions">
      <button @click="$emit('auto-pick')" class="btn btn-secondary">Auto-Pick All</button>
      <button @click="$emit('clear-colors')" class="btn btn-secondary">Clear</button>
      <button @click="$emit('activate-eyedropper')" class="btn btn-secondary">Eyedropper</button>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ColorPalette',
  props: {
    maxLayers: Number,
    colors: Object,
    selectedLayer: Number,
    pickedCount: Number
  },
  methods: {
    getColorDisplay(index) {
      if (this.colors[index]) {
        const [r, g, b] = this.colors[index];
        return `rgb(${r}, ${g}, ${b})`;
      }
      return '#f0f0f0';
    }
  }
}
</script>

<style scoped>
.color-palette {
  background: #f9f9f9;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
}

.palette-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(50px, 1fr));
  gap: 10px;
  margin-top: 15px;
}

.color-box {
  aspect-ratio: 1;
  border: 2px solid #ddd;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
  position: relative;
}

.color-box:hover {
  transform: scale(1.1);
  border-color: #2196F3;
}

.color-box.active {
  border-color: #4CAF50;
  border-width: 3px;
}

.color-index {
  position: absolute;
  bottom: 2px;
  right: 2px;
  background: white;
  padding: 1px 4px;
  border-radius: 3px;
  font-size: 10px;
  font-weight: bold;
}

.status {
  padding: 10px;
  background: #e3f2fd;
  border-radius: 6px;
  margin-top: 15px;
  font-size: 14px;
  color: #1976D2;
}

.actions {
  display: flex;
  gap: 10px;
  margin-top: 20px;
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

<template>
  <div class="controls">
    <div class="control-group">
      <label>Number of Layers (0 to n)</label>
      <input type="number" :value="maxLayers" @input="handleMaxLayersInput" min="1" max="50">
    </div>
    <div class="control-group">
      <label>Grid Size</label>
      <input type="number" :value="gridSize.x" @input="handleGridSizeInput('x', $event)" min="1" max="10">
      X
      <input type="number" :value="gridSize.y" @input="handleGridSizeInput('y', $event)" min="1" max="10">
    </div>
  </div>
</template>

<script>
export default {
  name: 'GridControls',
  props: {
    maxLayers: Number,
    gridSize: Object
  },
  emits: ['update:maxLayers', 'update-grid-size'],
  methods: {
    handleMaxLayersInput(event) {
      const value = event.target.value;
      if (value === '') {
        this.$emit('update:maxLayers', 0);
        return;
      }
      const parsedValue = parseInt(value, 10);
      if (!isNaN(parsedValue)) {
        this.$emit('update:maxLayers', parsedValue);
      }
    },
    handleGridSizeInput(axis, event) {
      const value = event.target.value;
      if (value === '') {
        this.$emit('update-grid-size', { axis, value: 0 });
        return;
      }
      const parsedValue = parseInt(value, 10);
      if (!isNaN(parsedValue)) {
        this.$emit('update-grid-size', { axis, value: parsedValue });
      }
    }
  }
}
</script>

<style scoped>
.controls {
  display: flex;
  gap: 20px;
  margin-bottom: 30px;
  flex-wrap: wrap;
}

.control-group {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

label {
  font-size: 14px;
  color: #666;
  font-weight: 500;
}

input[type="number"] {
  padding: 8px 12px;
  border: 2px solid #e0e0e0;
  border-radius: 6px;
  font-size: 16px;
  width: 120px;
}

input[type="number"]:focus {
  outline: none;
  border-color: #4CAF50;
}
</style>

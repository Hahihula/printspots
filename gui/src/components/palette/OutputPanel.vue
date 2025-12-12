<template>
  <div class="output-section">
    <h3>Output Preview</h3>
    <div class="output-preview">{{ output }}</div>

    <div class="actions">
      <button @click="$emit('download')" class="btn btn-primary">Download .{{ file_format }}</button>
      <button @click="$emit('copy')" class="btn btn-secondary">Copy</button>
    </div>
  </div>
</template>

<script>
export default {
  name: 'OutputPanel',
  props: {
    output: String,
    file_format: String
  }
}
</script>

<style scoped>
.output-section {
  background: #f9f9f9;
  border-radius: 8px;
  padding: 20px;
}

.output-preview {
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  padding: 15px;
  font-family: 'Courier New', monospace;
  font-size: 14px;
  max-height: 200px;
  overflow-y: auto;
  margin: 15px 0;
  white-space: pre;
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

.btn-primary {
  background: #4CAF50;
  color: white;
}

.btn-primary:hover {
  background: #45a049;
}

.btn-secondary {
  background: #2196F3;
  color: white;
  margin-right: 10px;
}

.btn-secondary:hover {
  background: #1976D2;
}

.actions {
  display: flex;
  gap: 10px;
  margin-top: 20px;
}
</style>

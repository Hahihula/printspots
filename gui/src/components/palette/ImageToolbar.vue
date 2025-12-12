<template>
  <div style="margin-bottom: 15px; display: flex; gap: 10px;">
    <button @click="$emit('rotate-left')" class="btn btn-secondary" title="Rotate 90° left">↺</button>
    <button @click="$emit('rotate-right')" class="btn btn-secondary" title="Rotate 90° right">↻</button>
    <button @click="$emit('flip-horizontal')" class_="btn btn-secondary" title="Flip horizontal">↔</button>
    <button @click="$emit('flip-vertical')" class="btn btn-secondary" title="Flip vertical">↕</button>
  </div>
</template>

<script>
export default {
  name: 'ImageToolbar',
}
</script>

<style scoped>
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

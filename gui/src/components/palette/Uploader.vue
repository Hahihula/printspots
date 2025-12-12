<template>
  <div class="upload-area"
       :class="{ 'has-image': hasImage }"
       @click="$refs.fileInput.click()" @drop.prevent="handleDrop" @dragover.prevent @dragenter.prevent>
    <div style="font-size: 48px; margin-bottom: 20px;"></div>
    <h3>Drop your scanned calibration grid here</h3>
    <p style="margin-top: 10px; color: #999;">or click to browse</p>
    <input ref="fileInput" type="file" accept="image/*" @change="handleFileSelect" style="display: none;">
  </div>
</template>

<script>
export default {
  name: 'Uploader',
  props: {
    hasImage: Boolean
  },
  methods: {
    handleDrop(e) {
      const files = e.dataTransfer.files;
      if (files.length > 0) {
        this.$emit('file-selected', files[0]);
      }
    },
    handleFileSelect(e) {
      const files = e.target.files;
      if (files.length > 0) {
        this.$emit('file-selected', files[0]);
      }
    }
  }
}
</script>

<style scoped>
.upload-area {
  border: 3px dashed #ccc;
  border-radius: 12px;
  padding: 40px;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s;
  margin-bottom: 30px;
}

.upload-area:hover {
  border-color: #4CAF50;
  background: #f9f9f9;
}

.upload-area.has-image {
  display: none;
}
</style>

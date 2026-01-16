<template>
  <div class="container">
    <h1>Grayscale Calibration Grid Picker</h1>
    <p class="subtitle">Extract color palette from your printed calibration grid</p>

    <GridControls :max-layers="maxLayers" :grid-size="gridSize" @update:max-layers="maxLayers = $event"
      @update-grid-size="updateGridSize" />

    <Uploader :has-image="!!imageUrl" @file-selected="loadImage" />

    <div v-if="imageUrl" class="main-content">
      <div class="image-section">
        <div class="instructions">
          <strong>Instructions:</strong> Click on each grid square to pick its color. The grid should match your printed
          calibration pattern ({{ gridSize.x }}×{{ gridSize.y }} squares, layers 0-{{ maxLayers }}).
        </div>

        <ImageToolbar @rotate-left="rotateLeft" @rotate-right="rotateRight" @flip-horizontal="flipHorizontal"
          @flip-vertical="flipVertical" />

        <div class="white-balance-controls">
          <button @click="autoWhiteBalance" class="btn btn-secondary">🎨 Auto White Balance</button>
          <button @click="activateWhiteBalancePicker" class="btn btn-secondary" 
                  :class="{ 'active': isWhiteBalancePickerActive }">
            {{ isWhiteBalancePickerActive ? '👆 Click on white pixel...' : '⚪ Set White Balance' }}
          </button>
          <button @click="resetFilters" class="btn btn-secondary" title="Reset all rotations, flips and white balance">🔄 Reset</button>
        </div>

        <Grid ref="grid" :image-url="imageUrl" :transformed-image-url="transformedImageUrl" :grid-ready="gridReady"
          :grid-cells="gridCells" :colors="colors" :grid-offset="gridOffset" :grid-scale="gridScale"
          :is-eyedropper-active="isEyedropperActive" :is-white-balance-picker-active="isWhiteBalancePickerActive"
          :hide-grid="isEyedropperActive || isWhiteBalancePickerActive" @image-load="onImageLoad"
          @pick-color="pickColor" @update-grid-offset="updateGridOffset" @update-grid-scale="updateGridScale"
          @auto-detect-grid="autoDetectGrid" @pick-color-with-eyedropper="pickColorWithEyedropper"
          @pick-white-balance-pixel="pickWhiteBalancePixel" />
      </div>

      <div class="sidebar">
        <ColorPalette :max-layers="maxLayers" :colors="colors" :selected-layer="selectedLayer"
          :picked-count="pickedCount" @update:selected-layer="selectedLayer = $event" @auto-pick="autoPick"
          @clear-colors="clearColors" @activate-eyedropper="activateEyedropper" />

        <div class="control-group" style="margin-bottom: 20px;">
          <button @click="savePalette" class="btn btn-primary" style="width: 100%; margin-bottom: 10px;">
            Save Palette & Continue
          </button>
        </div>

        <div class="control-group" style="margin-bottom: 20px;">
          <label class="checkbox-label">
            <input type="checkbox" v-model="skipSimilarColors"> Skip similar colors
          </label>
        </div>

        <div class="control-group" style="margin-bottom: 20px;">
          <label>Output Format (Debug)</label>
          <select v-model="outputFormat">
            <option value="ron">RON (New)</option>
            <option value="toml">TOML (Legacy)</option>
          </select>
        </div>

        <OutputPanel :output="generateOutput()" @download="downloadFile" @copy="copyToClipboard"
          :file_format="outputFormat" />
      </div>
    </div>
  </div>
</template>

<script>
import Uploader from './Uploader.vue';
import ImageToolbar from './ImageToolbar.vue';
import GridControls from './GridControls.vue';
import ColorPalette from './ColorPalette.vue';
import Grid from './Grid.vue';
import OutputPanel from './OutputPanel.vue';

export default {
  name: 'GrayscaleCalibration',
  components: {
    Uploader,
    ImageToolbar,
    GridControls,
    ColorPalette,
    OutputPanel,
    Grid
  },
  props: {
    initialMaxLayers: {
      type: Number,
      default: 15
    }
  },
  data() {
    return {
      maxLayers: this.initialMaxLayers,
      gridSize: { x: 4, y: 4 },
      imageUrl: null,
      transformedImageUrl: null,
      colors: {},
      selectedLayer: null,
      gridCells: [],
      gridReady: false,
      gridOffset: { x: 0, y: 0 },
      gridScale: 1.0,
      isEyedropperActive: false,
      isWhiteBalancePickerActive: false,
      imageFilters: {
        rotation: 0,
        flippedH: false,
        flippedV: false,
        wb: {
          r: 1.0,
          g: 1.0,
          b: 1.0
        }
      },
      skipSimilarColors: false,
      outputFormat: 'ron'
    };
  },
  computed: {
    pickedCount() {
      return Object.keys(this.colors).length;
    }
  },
  watch: {
    maxLayers() {
      this.updateGrid();
    },
    gridSize: {
      handler() {
        this.updateGrid();
      },
      deep: true
    },
    gridOffset: {
      handler() {
        this.updateGrid();
      },
      deep: true
    },
    gridScale() {
      this.updateGrid();
    }
  },
  mounted() {
    this.gridSize = {
      x: Math.ceil(Math.sqrt(this.maxLayers + 1)),
      y: Math.ceil(Math.sqrt(this.maxLayers + 1))
    };
  },
  methods: {
    loadImage(file) {
      const reader = new FileReader();
      reader.onload = (e) => {
        this.imageUrl = e.target.result;
        this.transformedImageUrl = null;
        this.colors = {};
        this.gridReady = false;
        
        // Reset filters for new image
        this.imageFilters = {
          rotation: 0,
          flippedH: false,
          flippedV: false,
          wb: { r: 1.0, g: 1.0, b: 1.0 }
        };

        // Auto white balance after image loads
        this.$nextTick(() => {
          this.autoWhiteBalance();
        });
      };
      reader.readAsDataURL(file);
    },
    onImageLoad() {
      this.$nextTick(() => {
        this.updateGrid();
        this.gridReady = true;
      });
    },
    updateGrid() {
      const img = this.$refs.grid?.$refs.calibrationImage;
      if (!img) return;

      const width = img.width * this.gridScale;
      const height = img.height * this.gridScale;
      const cellWidth = width / this.gridSize.x;
      const cellHeight = height / this.gridSize.y;

      this.gridCells = [];
      let layerIndex = 0;

      for (let row = 0; row < this.gridSize.y; row++) {
        for (let col = 0; col < this.gridSize.x; col++) {
          if (layerIndex <= this.maxLayers) {
            this.gridCells.push({
              row: row,
              col: col,
              x: col * cellWidth + this.gridOffset.x,
              y: row * cellHeight + this.gridOffset.y,
              width: cellWidth,
              height: cellHeight,
              layerIndex: layerIndex
            });
            layerIndex++;
          }
        }
      }
    },

    async autoPick() {
      // Pick all colors and wait for all of them to finish
      const pickPromises = [];
      for (let cell of this.gridCells) {
        pickPromises.push(this.pickColor(cell));
      }
      
      await Promise.all(pickPromises);
      
      // Wait a tiny bit for Vue to update data if needed
      this.$nextTick(() => {
        this.deduplicateSimilarColors();
      });
    },

    pickColor(cell) {
      return new Promise((resolve) => {
        const canvas = document.createElement('canvas');
        const ctx = canvas.getContext('2d');
        const img = this.$refs.grid?.$refs.calibrationImage;
        if (!img) { resolve(); return; }

        const imgSource = new Image();
        imgSource.onload = () => {
          canvas.width = imgSource.width;
          canvas.height = imgSource.height;
          ctx.drawImage(imgSource, 0, 0);

          const scaleX = imgSource.width / img.width;
          const scaleY = imgSource.height / img.height;

          const centerX = (cell.x + cell.width / 2) * scaleX / this.gridScale;
          const centerY = (cell.y + cell.height / 2) * scaleY / this.gridScale;

          const sampleSize = 10;
          const imageData = ctx.getImageData(
            Math.max(0, centerX - sampleSize / 2),
            Math.max(0, centerY - sampleSize / 2),
            sampleSize,
            sampleSize
          );

          const data = imageData.data;
          let r = 0, g = 0, b = 0, count = 0;

          for (let i = 0; i < data.length; i += 4) {
            r += data[i];
            g += data[i + 1];
            b += data[i + 2];
            count++;
          }

          r = Math.round(r / count);
          g = Math.round(g / count);
          b = Math.round(b / count);

          this.colors[cell.layerIndex] = [r, g, b];
          this.selectedLayer = cell.layerIndex;
          resolve();
        };
        imgSource.onerror = () => resolve();
        imgSource.src = this.transformedImageUrl || this.imageUrl;
      });
    },

    // Calculate perceptual color difference (simplified Delta E)
    colorDifference(color1, color2) {
      const [r1, g1, b1] = color1;
      const [r2, g2, b2] = color2;

      // Weighted Euclidean distance (approximates human perception)
      const rMean = (r1 + r2) / 2;
      const deltaR = r1 - r2;
      const deltaG = g1 - g2;
      const deltaB = b1 - b2;

      const weightR = 2 + rMean / 256;
      const weightG = 4.0;
      const weightB = 2 + (255 - rMean) / 256;

      return Math.sqrt(
        weightR * deltaR * deltaR +
        weightG * deltaG * deltaG +
        weightB * deltaB * deltaB
      );
    },

    deduplicateSimilarColors() {
      const SIMILARITY_THRESHOLD = 15;
      const WHITE_THRESHOLD = 200; // RGB average above this = white enough

      const layerIndices = Object.keys(this.colors)
        .map(k => parseInt(k))
        .sort((a, b) => a - b);

      if (layerIndices.length < 2) return;

      const toRemove = new Set();
      let firstWhiteLayer = null;
      let whiteCount = 0;

      // Find first "white enough" color
      for (let layer of layerIndices) {
        const [r, g, b] = this.colors[layer];
        const brightness = (r + g + b) / 3;
        if (brightness >= WHITE_THRESHOLD) {
          if (firstWhiteLayer === null) {
            firstWhiteLayer = layer;
          }
          break;
        }
      }

      // Remove all layers above first white (they're all white enough)
      if (firstWhiteLayer !== null) {
        for (let layer of layerIndices) {
          if (layer > firstWhiteLayer) {
            toRemove.add(layer);
            whiteCount++;
          }
        }
      }

      // Deduplicate similar colors (prefer lower layer count)
      if (this.skipSimilarColors) {
        for (let i = 0; i < layerIndices.length; i++) {
          if (toRemove.has(layerIndices[i])) continue;

          for (let j = i + 1; j < layerIndices.length; j++) {
            if (toRemove.has(layerIndices[j])) continue;

            const layer1 = layerIndices[i];
            const layer2 = layerIndices[j];
            const diff = this.colorDifference(this.colors[layer1], this.colors[layer2]);

            if (diff < SIMILARITY_THRESHOLD) {
              // Always remove the higher layer count (keep lower)
              toRemove.add(layer2);
            }
          }
        }
      }

      // Apply removals and report
      toRemove.forEach(layer => delete this.colors[layer]);

      if (toRemove.size > 0) {
        const duplicateCount = toRemove.size - whiteCount;
        let message = `Auto-pick complete! Removed ${toRemove.size} colors`;
        if (whiteCount > 0 && duplicateCount > 0) {
          message += ` (${whiteCount} above white threshold, ${duplicateCount} duplicates)`;
        } else if (whiteCount > 0) {
          message += ` (above white threshold)`;
        } else {
          message += ` (duplicates)`;
        }
        console.log(message, Array.from(toRemove));
        alert(message + '.');
      }
    },
    autoDetectGrid() {
      const img = this.$refs.grid?.$refs.calibrationImage;
      if (!img) return;

      this.gridOffset = { x: 10, y: 10 };
      this.gridScale = 0.95;
      this.updateGrid();
    },
    updateGridSize({ axis, value }) {
      this.gridSize[axis] = value;
    },
    updateGridOffset({ axis, value }) {
      this.gridOffset[axis] = value;
    },
    updateGridScale(value) {
      this.gridScale = value;
    },
    activateEyedropper() {
      if (this.selectedLayer === null) {
        alert('Please select a color square first.');
        return;
      }
      this.isEyedropperActive = true;
    },
    pickColorWithEyedropper(event) {
      if (!this.isEyedropperActive) return;

      const canvas = document.createElement('canvas');
      const ctx = canvas.getContext('2d');
      const img = this.$refs.grid?.$refs.calibrationImage;
      if (!img) return;

      canvas.width = img.naturalWidth;
      canvas.height = img.naturalHeight;
      ctx.drawImage(img, 0, 0);

      const rect = img.getBoundingClientRect();
      const x = event.clientX - rect.left;
      const y = event.clientY - rect.top;

      const scaleX = img.naturalWidth / img.width;
      const scaleY = img.naturalHeight / img.height;

      const imageData = ctx.getImageData(x * scaleX, y * scaleY, 1, 1);
      const [r, g, b] = imageData.data;

      this.colors[this.selectedLayer] = [r, g, b];
      this.isEyedropperActive = false;
    },
    
    // --- Improved Image Pipeline & Filters ---

    applyImagePipeline() {
      if (!this.imageUrl) return;
      console.log('Applying image pipeline:', this.imageFilters);

      const img = new Image();
      img.onload = () => {
        const canvas = document.createElement('canvas');
        const ctx = canvas.getContext('2d');

        // 1. Determine canvas dimensions based on rotation
        if (this.imageFilters.rotation % 180 === 90) {
          canvas.width = img.height;
          canvas.height = img.width;
        } else {
          canvas.width = img.width;
          canvas.height = img.height;
        }

        // 2. Apply Transformations (Rotation & Flip)
        ctx.save();
        ctx.translate(canvas.width / 2, canvas.height / 2);
        ctx.rotate((this.imageFilters.rotation * Math.PI) / 180);
        ctx.scale(this.imageFilters.flippedH ? -1 : 1, this.imageFilters.flippedV ? -1 : 1);
        ctx.drawImage(img, -img.width / 2, -img.height / 2);
        ctx.restore();

        // 3. Apply White Balance (if any filters set)
        const wb = this.imageFilters.wb;
        if (wb.r !== 1.0 || wb.g !== 1.0 || wb.b !== 1.0) {
          console.log('Scaling colors:', wb);
          const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
          const data = imageData.data;
          for (let i = 0; i < data.length; i += 4) {
            data[i] = Math.min(255, Math.max(0, Math.round(data[i] * wb.r)));
            data[i + 1] = Math.min(255, Math.max(0, Math.round(data[i + 1] * wb.g)));
            data[i + 2] = Math.min(255, Math.max(0, Math.round(data[i + 2] * wb.b)));
          }
          ctx.putImageData(imageData, 0, 0);
        }

        this.transformedImageUrl = canvas.toDataURL('image/png');
        this.gridReady = false;
        this.$nextTick(() => {
          this.updateGrid();
          this.gridReady = true;
          console.log('Pipeline update complete');
        });
      };
      img.onerror = (err) => console.error('Failed to load original image in pipeline:', err);
      img.src = this.imageUrl;
    },

    autoWhiteBalance() {
      if (!this.imageUrl) return;
      console.log('Calculating auto white balance...');

      const img = new Image();
      img.onload = () => {
        const canvas = document.createElement('canvas');
        const ctx = canvas.getContext('2d');
        canvas.width = Math.min(img.width, 1000); // Scale down for speed
        canvas.height = (canvas.width / img.width) * img.height;
        ctx.drawImage(img, 0, 0, canvas.width, canvas.height);

        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        const data = imageData.data;

        // Better AWB: use 99th percentile to find "white"
        const rVals = [], gVals = [], bVals = [];
        // Sample 10000 pixels or total pixels
        const step = Math.max(4, Math.floor(data.length / (10000 * 4)) * 4);
        for (let i = 0; i < data.length; i += step) {
            rVals.push(data[i]);
            gVals.push(data[i+1]);
            bVals.push(data[i+2]);
        }
        
        rVals.sort((a, b) => a - b);
        gVals.sort((a, b) => a - b);
        bVals.sort((a, b) => a - b);
        
        // Use 99.5th percentile
        const pIdx = Math.floor(rVals.length * 0.995);
        const refR = rVals[pIdx] || 255;
        const refG = gVals[pIdx] || 255;
        const refB = bVals[pIdx] || 255;
        
        console.log('Reference white point:', refR, refG, refB);

        this.imageFilters.wb = {
          r: refR > 0 ? 255 / refR : 1.0,
          g: refG > 0 ? 255 / refG : 1.0,
          b: refB > 0 ? 255 / refB : 1.0
        };
        
        this.applyImagePipeline();
      };
      img.src = this.imageUrl;
    },

    activateWhiteBalancePicker() {
      this.isWhiteBalancePickerActive = true;
      this.isEyedropperActive = false;
    },

    pickWhiteBalancePixel(event) {
      if (!this.isWhiteBalancePickerActive || !this.imageUrl) return;

      const img = this.$refs.grid?.$refs.calibrationImage;
      if (!img) return;

      const canvas = document.createElement('canvas');
      const ctx = canvas.getContext('2d');
      const imgSource = new Image();
      
      imgSource.onload = () => {
        canvas.width = imgSource.width;
        canvas.height = imgSource.height;
        ctx.drawImage(imgSource, 0, 0);

        const rect = img.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;

        const scaleX = imgSource.width / img.width;
        const scaleY = imgSource.height / img.height;

        const imageData = ctx.getImageData(x * scaleX, y * scaleY, 1, 1);
        const [r, g, b] = imageData.data;
        
        console.log('Sampled pixel for manual WB:', r, g, b);

        // Additive WB: calculate further scaling needed to reach 255
        const furtherScaleR = r > 0 ? 255 / r : 1.0;
        const furtherScaleG = g > 0 ? 255 / g : 1.0;
        const furtherScaleB = b > 0 ? 255 / b : 1.0;

        this.imageFilters.wb = {
          r: this.imageFilters.wb.r * furtherScaleR,
          g: this.imageFilters.wb.g * furtherScaleG,
          b: this.imageFilters.wb.b * furtherScaleB
        };
        
        console.log('New WB factors:', this.imageFilters.wb);

        this.isWhiteBalancePickerActive = false;
        this.applyImagePipeline();
      };
      imgSource.src = this.transformedImageUrl || this.imageUrl;
    },

    resetFilters() {
      this.imageFilters = {
        rotation: 0,
        flippedH: false,
        flippedV: false,
        wb: { r: 1.0, g: 1.0, b: 1.0 }
      };
      this.transformedImageUrl = null;
      this.applyImagePipeline();
    },

    rotateLeft() {
      this.imageFilters.rotation = (this.imageFilters.rotation - 90 + 360) % 360;
      this.applyImagePipeline();
    },

    rotateRight() {
      this.imageFilters.rotation = (this.imageFilters.rotation + 90) % 360;
      this.applyImagePipeline();
    },

    flipHorizontal() {
      this.imageFilters.flippedH = !this.imageFilters.flippedH;
      this.applyImagePipeline();
    },

    flipVertical() {
      this.imageFilters.flippedV = !this.imageFilters.flippedV;
      this.applyImagePipeline();
    },

    clearColors() {
      this.colors = {};
    },
    getColorDisplay(index) {
      if (this.colors[index]) {
        const [r, g, b] = this.colors[index];
        return `rgb(${r}, ${g}, ${b})`;
      }
      return '#f0f0f0';
    },
    savePalette() {
      const colors = [];
      const layer_counts = [];

      // Only export picked colors (sorted by layer index)
      const pickedLayers = Object.keys(this.colors)
        .map(k => parseInt(k))
        .sort((a, b) => a - b);

      for (let layer of pickedLayers) {
        layer_counts.push(layer);
        colors.push(this.colors[layer]);
      }

      this.$emit('save', { colors, layer_counts });
    },
    generateOutput() {
      if (this.outputFormat === 'ron') {
        return this.generateRonOutput();
      } else {
        return this.generateTomlOutput();
      }
    },
    generateTomlOutput() {
      let output = 'colors = [';
      const colorArrays = [];

      // Only export picked colors
      const pickedLayers = Object.keys(this.colors)
        .map(k => parseInt(k))
        .sort((a, b) => a - b);

      for (let layer of pickedLayers) {
        colorArrays.push(`[${this.colors[layer].join(', ')}]`);
      }

      output += colorArrays.join(', ') + ']\n';
      output += 'layer_counts = [';
      output += pickedLayers.join(', ') + ']';

      return output;
    },
    generateRonOutput() {
      let output = '(\n';
      output += '    colors: [\n';

      // Only export picked colors
      const pickedLayers = Object.keys(this.colors)
        .map(k => parseInt(k))
        .sort((a, b) => a - b);

      for (let layer of pickedLayers) {
        const [r, g, b] = this.colors[layer];
        output += `        (${r}, ${g}, ${b}),\n`;
      }

      output += '    ],\n';
      output += '    layer_counts: [\n';
      output += `        ${pickedLayers.join(', ')}\n`;
      output += '    ],\n';
      output += ')';

      return output;
    },
    downloadFile() {
      const content = this.generateOutput();
      const blob = new Blob([content], { type: 'text/plain' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `grayscale_palette.${this.outputFormat}`;
      a.click();
      URL.revokeObjectURL(url);
    },
    async copyToClipboard() {
      const content = this.generateOutput();
      try {
        await navigator.clipboard.writeText(content);
        alert('Copied to clipboard!');
      }
      catch (err) {
        alert('Failed to copy to clipboard');
      }
    }
  }
}
</script>

<style scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

.container {
  max-width: 100%;
  width: 100%;
  margin: 0;
  background: transparent;
  border-radius: 0;
  box-shadow: none;
  padding: 20px;
  color: #fff;
  overflow-y: auto;
  max-height: calc(100vh - 60px); /* Account for header */
  height: 100%;
}

h1 {
  color: #fff;
  margin-bottom: 10px;
}

.subtitle {
  color: #aaa;
  margin-bottom: 30px;
}

.controls {
  display: flex;
  gap: 20px;
  margin-bottom: 30px;
  flex-wrap: wrap;
  align-items: flex-end;
}

.control-group {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

select {
  padding: 8px 12px;
  border: 1px solid #555;
  border-radius: 6px;
  font-size: 16px;
  background: #f0f0f0;
  color: #1a1a1a;
  min-width: 120px;
  color-scheme: light; /* Force light mode for dropdown */
}

select:focus {
  outline: none;
  border-color: #4CAF50;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  color: #e0e0e0;
  font-size: 14px;
}

.checkbox-label input {
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.control-group label {
  font-size: 14px;
  color: #e0e0e0;
  font-weight: 500;
}

input[type="number"] {
  padding: 8px 12px;
  border: 1px solid #444;
  border-radius: 6px;
  font-size: 16px;
  width: 120px;
  background: #333;
  color: white;
}

input[type="number"]:focus {
  outline: none;
  border-color: #4CAF50;
}

.upload-area {
  border: 3px dashed #555;
  border-radius: 12px;
  padding: 40px;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s;
  margin-bottom: 30px;
  background: rgba(255, 255, 255, 0.02);
}

.upload-area:hover {
  border-color: #4CAF50;
  background: rgba(255, 255, 255, 0.05);
}

.upload-area.has-image {
  display: none;
}

.main-content {
  display: flex;
  gap: 30px;
  flex-direction: column;
  /* Stacking on mobile */
}

@media (min-width: 768px) {
  .main-content {
    flex-direction: row;
  }
}

.image-section {
  flex: 1;
}

.image-container {
  position: relative;
  display: inline-block;
  border: 2px solid #444;
  border-radius: 8px;
  overflow: hidden;
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
  border: 1px solid rgba(100, 100, 255, 0.5);
  cursor: pointer;
  pointer-events: all;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.grid-cell:hover {
  border: 2px solid #2196F3;
  background: rgba(33, 150, 243, 0.2);
}

.grid-cell.picked {
  border: 2px solid #4CAF50;
  background: rgba(76, 175, 80, 0.2);
}

.cell-label {
  background: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: bold;
}

.sidebar {
  width: 100%;
}

@media (min-width: 768px) {
  .sidebar {
    width: 300px;
    /* Reduced from 350px just in case */
  }
}

.color-palette {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
}

.palette-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(40px, 1fr));
  gap: 8px;
  margin-top: 15px;
}

.color-box {
  aspect-ratio: 1;
  border: 2px solid #555;
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
  background: rgba(0, 0, 0, 0.8);
  color: white;
  /* Ensure text is visible */
  padding: 1px 4px;
  border-radius: 3px;
  font-size: 10px;
  font-weight: bold;
}

.output-section {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  padding: 20px;
}

.output-preview {
  background: #1a1a1a;
  border: 1px solid #555;
  border-radius: 6px;
  padding: 15px;
  font-family: 'Courier New', monospace;
  font-size: 12px;
  max-height: 200px;
  overflow-y: auto;
  margin: 15px 0;
  white-space: pre;
  color: #e0e0e0;
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

.status {
  padding: 10px;
  background: rgba(25, 118, 210, 0.2);
  border-radius: 6px;
  margin-top: 15px;
  font-size: 14px;
  color: #90caf9;
}

.instructions {
  background: rgba(255, 152, 0, 0.1);
  border-left: 4px solid #ff9800;
  padding: 15px;
  margin-bottom: 20px;
  border-radius: 4px;
  color: white;
}

.grid-adjust {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid #444;
  border-radius: 6px;
  padding: 10px;
  margin-top: 10px;
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
  border: 1px solid #444;
  border-radius: 4px;
  background: #333;
  color: white;
}
</style>

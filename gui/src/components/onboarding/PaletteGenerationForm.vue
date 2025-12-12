<template>
  <div class="palette-generation-form-wrapper">
    <div class="palette-generation-form">
      <!-- Calibration Step -->
      <div v-if="currentStep === 'calibration'" class="text-center py-8">
        <h3 class="text-2xl font-bold text-white mb-6">Filament Calibration</h3>
        <p class="text-text-muted mb-6">
          To ensure accurate color mixing, we need to calibrate your filament.
        </p>

        <!-- Filament Form -->
        <div class="text-left mb-6">
          <!-- <div class="flex justify-between items-center mb-2">
          <label class="font-bold">Filaments (Min 2)</label>
          <button @click="addFilament" class="text-sm text-primary hover:text-white">+ Add Filament</button>
        </div> -->

          <div v-for="(filament, index) in localCalibrationData.filaments" :key="index"
            class="p-4 bg-white/5 rounded-lg mb-3 relative group">
            <button v-if="localCalibrationData.filaments.length > 2" @click="removeFilament(index)"
              class="absolute top-2 right-2 text-red-500 hover:text-red-400 opacity-0 group-hover:opacity-100 transition-opacity">✕</button>

            <div class="grid grid-cols-2 gap-3">
              <div class="form-group mb-0">
                <label class="text-xs text-text-muted">Brand</label>
                <input v-model="filament.brand" type="text" class="form-input text-sm py-1"
                  placeholder="e.g. Prusament" />
              </div>
              <div class="form-group mb-0">
                <label class="text-xs text-text-muted">Color</label>
                <input v-model="filament.color" type="text" class="form-input text-sm py-1" placeholder="e.g. Black" />
              </div>
            </div>
          </div>
        </div>

        <div v-if="generatedPath" class="bg-green-500/10 border border-green-500 rounded-lg p-4 mb-6 text-left">
          <p class="text-green-400 font-bold mb-2">Calibration Object Generated!</p>
          <div class="flex items-center gap-2 mb-2">
            <p class="text-sm text-text-muted break-all flex-1">{{ generatedPath }}</p>
            <button @click="openFolder(generatedPath)"
              class="text-xs bg-primary/20 hover:bg-primary/30 text-primary px-3 py-2 rounded-lg transition-colors font-medium"
              title="Show in Folder">
              📂 Open Folder
            </button>
          </div>
          <p class="text-sm">Open this 3MF file in your slicer and print it.</p>
        </div>

        <div v-if="isGenerating" class="text-primary animate-pulse mb-6">Generating...</div>
        <div v-if="errorMsg" class="text-secondary mb-6">{{ errorMsg }}</div>

        <div class="flex justify-between items-center gap-3">
          <button v-if="showCancel" @click="$emit('cancel')"
            class="px-6 py-3 bg-white/10 hover:bg-white/20 rounded-lg transition-colors font-medium">
            Cancel
          </button>
          <button v-if="!generatedPath" @click="generateCalibration" :disabled="isGenerating || !canGenerate"
            class="btn-cmyk px-6 py-3 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed flex-1 font-bold">
            Generate 3MF
          </button>
          <button v-else @click="goToPaletteStep" class="btn-primary px-6 py-3 rounded-lg flex-1 font-bold">
            I've Printed It - Continue
          </button>
        </div>
      </div>

      <!-- Palette Creation Step -->
      <div v-if="currentStep === 'palette'">
        <GrayscaleCalibration :initial-max-layers="localCalibrationData.max_layers" @save="handlePaletteSave" />
      </div>

      <!-- Palette Name Modal -->
      <PaletteNameModal :show="showNameModal" @save="savePaletteWithName" @cancel="showNameModal = false" />
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GrayscaleCalibration from '../palette/GrayscaleCalibration.vue';
import PaletteNameModal from '../modals/PaletteNameModal.vue';

const props = defineProps({
  printerProfile: {
    type: Object,
    required: true
  },
  initialCalibrationData: {
    type: Object,
    default: () => ({
      filaments: [
        { brand: 'NoName', color: 'Black', material: 'PLA' },
        { brand: 'NoName', color: 'White', material: 'PLA' }
      ],
      base_thickness: 1.0,
      layer_thickness: 0.1,
      square_size: 10.0,
      max_layers: 19
    })
  },
  showCancel: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['save', 'cancel']);

const currentStep = ref('calibration');
const localCalibrationData = ref({ ...props.initialCalibrationData });
const isGenerating = ref(false);
const generatedPath = ref(null);
const errorMsg = ref(null);
const showNameModal = ref(false);
const pendingPaletteData = ref(null);

const canGenerate = computed(() => {
  return localCalibrationData.value.filaments.every(f => f.brand && f.color);
});

function addFilament() {
  localCalibrationData.value.filaments.push({ brand: '', color: '', material: 'PLA' });
}

function removeFilament(index) {
  if (localCalibrationData.value.filaments.length > 2) {
    localCalibrationData.value.filaments.splice(index, 1);
  }
}

async function openFolder(path) {
  try {
    await invoke('show_in_folder', { path });
  } catch (e) {
    console.error("Failed to open folder:", e);
    alert("Failed to open folder: " + e);
  }
}

async function generateCalibration() {
  isGenerating.value = true;
  errorMsg.value = null;
  try {
    const path = await invoke('generate_calibration', {
      profile: props.printerProfile,
      settings: localCalibrationData.value
    });
    generatedPath.value = path;
  } catch (e) {
    console.error(e);
    errorMsg.value = "Failed to generate: " + e;
  } finally {
    isGenerating.value = false;
  }
}

function goToPaletteStep() {
  currentStep.value = 'palette';
}

function handlePaletteSave(paletteData) {
  pendingPaletteData.value = paletteData;
  showNameModal.value = true;
}

async function savePaletteWithName(name) {
  try {
    await invoke('save_palette', {
      data: pendingPaletteData.value,
      name: name
    });

    showNameModal.value = false;
    emit('save', { paletteData: pendingPaletteData.value, name });
  } catch (e) {
    console.error(e);
    alert("Failed to save palette: " + e);
  }
}
</script>
<style scoped>
.palette-generation-form-wrapper {
  display: flex;
  justify-content: center;
  width: 100%;
}

.palette-generation-form {
  max-width: 900px;
  width: 100%;
  padding: 0 1rem;
}
</style>
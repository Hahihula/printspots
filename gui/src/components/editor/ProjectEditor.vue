<template>
  <div class="project-editor h-full flex flex-col">
    <!-- Show Completion Screen if 3MF generated -->
    <CompletionScreen v-if="project.meshStats" :meshStats="project.meshStats" :layerThickness="project.layer_thickness"
      @start-new-project="handleStartNewProject" />

    <!-- Show Editor if no 3MF yet -->
    <template v-else>
      <!-- Editor specific actions toolbar -->
      <div class="editor-toolbar h-10 bg-dark-lighter border-b border-white/5 px-4 flex items-center justify-between">
        <div class="flex items-center gap-2">
          <span class="text-xs text-text-muted font-bold">{{ project.name }}</span>
        </div>
        <div class="flex items-center gap-2">
          <button v-if="project.sourceImageUrl" @click="projectStore.loadSourceImage()"
            class="text-xs text-primary hover:text-primary/80 flex items-center gap-1 transition-colors">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
            Open Another Image
          </button>
          <button v-if="project.sourceImageUrl" @click="handleGeneratePrediction"
            :disabled="projectStore.isGeneratingPrediction"
            class="text-xs text-secondary hover:text-secondary/80 flex items-center gap-1 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
            <svg v-if="!projectStore.isGeneratingPrediction" xmlns="http://www.w3.org/2000/svg" class="h-4 w-4"
              fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
            <svg v-else class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
              </path>
            </svg>
            {{ projectStore.isGeneratingPrediction ? 'Generating...' : 'Generate Prediction' }}
          </button>
          <button @click="projectStore.closeProject()"
            class="text-xs text-red-400 hover:text-red-300 flex items-center gap-1 transition-colors">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
            Close
          </button>
        </div>
      </div>

      <SplitLayout>
        <template #left>
          <div class="h-full w-full relative overflow-hidden bg-dark">
            <!-- Canvas Editor -->
            <ImageCanvas v-if="project.sourceImageUrl" :imageUrl="project.sourceImageUrl" ref="imageCanvasRef" />

            <!-- Empty State -->
            <div v-else class="h-full flex items-center justify-center p-10 cursor-pointer"
              @click="projectStore.loadSourceImage()">
              <div
                class="text-center p-10 border-2 border-dashed border-white/10 hover:border-primary/30 rounded-xl transition-colors">
                <div class="mb-4 text-primary/50">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                  </svg>
                </div>
                <p class="mb-1 font-bold">No Image Selected</p>
                <p class="text-xs opacity-50 mb-4">Click to Open or Drag & Drop Image Here</p>
                <button class="btn-primary px-4 py-2 rounded text-sm">Open Image</button>
              </div>
            </div>
          </div>
        </template>

        <template #right>
          <div class="h-full relative flex items-center justify-center text-text-muted bg-black/20 p-4">
            <!-- Prediction Display -->
            <div v-if="project.predictionImageUrl" class="h-full w-full flex flex-col">
              <div class="flex-1 overflow-auto flex items-center justify-center">
                <img :src="project.predictionImageUrl" alt="Prediction" class="max-w-full max-h-full object-contain" />
              </div>
              <!-- Generate 3MF Button -->
              <div class="mt-4">
                <button @click="handleGenerate3MF" :disabled="projectStore.isGenerating3MF"
                  class="w-full px-4 py-3 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors flex items-center justify-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed">
                  <svg v-if="!projectStore.isGenerating3MF" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5"
                    fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
                  </svg>
                  <svg v-else class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none"
                    viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor"
                      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                    </path>
                  </svg>
                  {{ projectStore.isGenerating3MF ? 'Generating...' : 'Generate 3MF' }}
                </button>
              </div>
            </div>

            <!-- Empty State / Generate Prediction -->
            <div v-else class="text-center">
              <div class="mb-4 text-primary/50">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M13 10V3L4 14h7v7l9-11h-7z" />
                </svg>
              </div>
              <p class="mb-4 font-bold">Ready to Generate Prediction</p>
              <p class="text-xs opacity-50 mb-4">Click the button in the toolbar or below</p>
            </div>
          </div>
        </template>
      </SplitLayout>
    </template>
  </div>
</template>

<script setup>
import { ref, watch, onMounted } from 'vue';
import SplitLayout from './SplitLayout.vue';
import ImageCanvas from './ImageCanvas.vue';
import CompletionScreen from './CompletionScreen.vue';
import { useProjectStore } from '../../stores/project';
import { usePaletteStore } from '../../stores/palette';
import { usePrinterProfileStore } from '../../stores/printerProfile';

const props = defineProps(['project']);
const projectStore = useProjectStore();
const paletteStore = usePaletteStore();
const profileStore = usePrinterProfileStore();

const imageCanvasRef = ref(null);

onMounted(() => {
  // Set the canvas ref in the store so generatePrediction can access it
  if (imageCanvasRef.value) {
    projectStore.setImageCanvasRef(imageCanvasRef.value);
  }
});

watch(imageCanvasRef, (newRef) => {
  if (newRef) {
    projectStore.setImageCanvasRef(newRef);
  }
});

// Watch for palette/profile changes and auto-regenerate if prediction exists
watch(() => paletteStore.activePaletteId, async () => {
  if (props.project?.predictionImageUrl && !projectStore.isGeneratingPrediction) {
    console.log('Palette changed - regenerating prediction...');
    await handleGeneratePrediction();
  }
});

watch(() => profileStore.activeProfileId, async () => {
  if (props.project?.predictionImageUrl && !projectStore.isGeneratingPrediction) {
    console.log('Printer profile changed - regenerating prediction...');
    await handleGeneratePrediction();
  }
});

async function handleGeneratePrediction() {
  try {
    await projectStore.generatePrediction();
  } catch (err) {
    alert(`Failed to generate prediction: ${err.message}`);
  }
}

async function handleGenerate3MF() {
  try {
    await projectStore.generate3MF(false); // flat_top = false
  } catch (err) {
    alert(`Failed to generate 3MF: ${err.message}`);
  }
}

function handleStartNewProject() {
  projectStore.closeProject();
}
</script>

<style scoped>
.bg-dots {
  background-image: radial-gradient(#ffffff 1px, transparent 1px);
  background-size: 20px 20px;
  background-color: var(--dark-lighter);
  /* opacity: 0.05 targets dots color effectively if combined */
}
</style>

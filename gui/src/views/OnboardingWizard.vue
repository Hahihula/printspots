<template>
  <div class="onboarding-wizard">
    <!-- Main Wizard Content -->
    <div class="wizard-content">

      <!-- Header / Progress -->
      <div class="wizard-header relative">
        <div class="absolute top-0 right-0">
          <button @click="resetWizard" class="text-xs text-text-muted hover:text-white transition-colors">
            Reset Wizard
          </button>
        </div>
        <h2 class="text-3xl font-bold text-white mb-2">{{ currentStepTitle }}</h2>
        <p class="text-text-muted">{{ currentStepDesc }}</p>

        <div class="flex gap-2 mt-6">
          <div v-for="i in 2" :key="i" class="h-1 flex-1 rounded-full transition-colors duration-500"
            :class="i <= stepIndex ? 'bg-gradient-to-r from-primary to-secondary' : 'bg-white/10'">
          </div>
        </div>
      </div>

      <!-- Step 1: Printer Setup -->
      <div v-if="step === 'printer'" class="panel section-enter">
        <PrinterConfigForm :initial-data="printerData" @save="savePrinterAndContinue" />
      </div>

      <!-- Step 2: Palette Generation (Calibration + Palette) -->
      <div v-if="step === 'palette'" class="panel section-enter">
        <PaletteGenerationForm :printer-profile="savedPrinterProfile" :initial-calibration-data="calibrationData"
          @save="onPaletteSave" />
      </div>

    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed, watch } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { usePrinterProfileStore } from '../stores/printerProfile';
import PrinterConfigForm from '../components/onboarding/PrinterConfigForm.vue';
import PaletteGenerationForm from '../components/onboarding/PaletteGenerationForm.vue';
import { v4 as uuidv4 } from 'uuid';

const router = useRouter();
const profileStore = usePrinterProfileStore();

const step = ref('printer');
const printerData = ref({
  id: uuidv4(),
  name: '',
  description: '',
  bed_width: 220,
  bed_depth: 220,
  nozzle_diameter: 0.4,
  min_layer_height: 0.1
});

const savedPrinterProfile = ref(null);

const calibrationData = ref({
  filaments: [
    { brand: '', color: 'Black', material: 'PLA' },
    { brand: '', color: 'White', material: 'PLA' }
  ],
  base_thickness: 1.0,
  layer_thickness: 0.1,
  square_size: 10.0,
  max_layers: 19
});

// Wizard Steps Mapping
const stepsOrder = ['printer', 'palette'];
const stepIndex = computed(() => {
  return stepsOrder.indexOf(step.value) + 1;
});

const currentStepTitle = computed(() => {
  switch (step.value) {
    case 'printer': return "Let's set up your printer";
    case 'palette': return "Create Palette";
    default: return "";
  }
});

const currentStepDesc = computed(() => {
  switch (step.value) {
    case 'printer': return "Tell us about your machine so we can optimize the generation.";
    case 'palette': return "Calibrate your filaments and scan the result.";
    default: return "";
  }
});

onMounted(async () => {
  await profileStore.init();
  await loadProgress();
});

let saveTimeout = null;
const performSave = async () => {
  // Ensure ID exists before saving progress
  if (!printerData.value.id) {
    printerData.value.id = uuidv4();
  }

  const progress = {
    step: step.value,
    printer_data: printerData.value,
    calibration_data: calibrationData.value,
    saved_printer_profile: savedPrinterProfile.value
  };
  try {
    await invoke('save_wizard_progress', { progress });
  } catch (e) {
    console.error("Failed to save progress", e);
  }
};

watch([step, printerData, calibrationData], () => {
  if (saveTimeout) clearTimeout(saveTimeout);
  saveTimeout = setTimeout(performSave, 1000);
}, { deep: true });

async function loadProgress() {
  try {
    const progress = await invoke('load_wizard_progress');
    if (progress) {
      step.value = progress.step;
      if (progress.printer_data) printerData.value = progress.printer_data;
      if (progress.calibration_data) calibrationData.value = progress.calibration_data;
      if (progress.saved_printer_profile) savedPrinterProfile.value = progress.saved_printer_profile;
    } else {
      // Pre-selection logic: if no progress, check for existing profiles
      if (profileStore.hasProfiles) {
        const latest = profileStore.activeProfile || profileStore.profiles[profileStore.profiles.length - 1];
        if (latest) {
          printerData.value = { ...latest };
        }
      }
    }
  } catch (e) {
    console.error("Failed to load progress", e);
  }
}

async function resetWizard() {
  if (!confirm("Are you sure you want to reset the wizard? All progress will be lost.")) return;

  try {
    await invoke('clear_wizard_progress');

    // Reset local state
    step.value = 'printer';
    printerData.value = {
      id: uuidv4(),
      name: '',
      description: '',
      bed_width: 220,
      bed_depth: 220,
      nozzle_diameter: 0.4,
      min_layer_height: 0.1
    };
    calibrationData.value = {
      filaments: [
        { brand: '', color: 'Black', material: 'PLA' },
        { brand: '', color: 'White', material: 'PLA' }
      ],
      base_thickness: 1.0,
      layer_thickness: 0.1,
      square_size: 10.0,
      max_layers: 19
    };
    savedPrinterProfile.value = null;

  } catch (e) {
    console.error("Failed to reset progress", e);
  }
}

async function savePrinterAndContinue(printerDataFromForm) {
  // Set default layer thickness from printer min layer height
  if (printerDataFromForm.min_layer_height) {
    calibrationData.value.layer_thickness = printerDataFromForm.min_layer_height;
  }

  const profile = { ...printerDataFromForm };
  if (!profile.id) {
    profile.id = uuidv4();
  }
  await profileStore.saveProfile(profile);

  savedPrinterProfile.value = profile;
  printerData.value = printerDataFromForm;
  step.value = 'palette';
}

async function onPaletteSave() {
  try {
    alert("Palette saved successfully!");
    await invoke('clear_wizard_progress');
    finalizeWizard();
  } catch (e) {
    console.error(e);
    alert("Failed to complete wizard: " + e);
  }
}

function finalizeWizard() {
  router.push('/');
}
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.section-enter {
  animation: slideUp 0.5s ease-out;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Ensure the wizard takes full viewport and scrolls internally */
.onboarding-wizard {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  overflow-x: hidden;
  background-color: var(--dark);
}

.wizard-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  min-height: 100%;
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}

.wizard-header {
  margin-bottom: 3rem;
  text-align: center;
}
</style>

<template>
  <div class="add-palette-view">
    <div class="max-w-2xl mx-auto p-8">
      <div class="mb-8 text-center">
        <h1 class="text-3xl font-bold text-white mb-2">Add New Palette</h1>
        <p class="text-text-muted">Generate a color palette by calibrating your filaments.</p>
      </div>

      <PaletteGenerationForm v-if="printerProfile" :printer-profile="printerProfile" :show-cancel="true"
        @save="handleSave" @cancel="handleCancel" />

      <div v-else class="text-center text-text-muted">
        <p>No printer profile found. Please add a printer first.</p>
        <button @click="router.push('/add-printer')" class="mt-4 btn-primary px-6 py-2 rounded">
          Add Printer
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import { useRouter } from 'vue-router';
import { usePrinterProfileStore } from '../stores/printerProfile';
import { usePaletteStore } from '../stores/palette';
import { invoke } from '@tauri-apps/api/core';
import PaletteGenerationForm from '../components/onboarding/PaletteGenerationForm.vue';

const router = useRouter();
const profileStore = usePrinterProfileStore();
const paletteStore = usePaletteStore();

const printerProfile = computed(() => {
  return profileStore.activeProfile || profileStore.profiles[0];
});

async function handleSave({ paletteData, name }) {
  try {
    // Reload palettes to get the newly created one
    await paletteStore.loadPalettes();
    
    // Find and set the newly created palette as active
    const newPalette = paletteStore.palettes.find(p => p.id === name.replace(/ /g, '_'));
    if (newPalette) {
      paletteStore.setActivePalette(newPalette.id);
    }
    
    alert("Palette saved successfully!");
    router.push('/');
  } catch (e) {
    console.error('Failed to save palette:', e);
    alert('Failed to save palette: ' + e.message);
  }
}

function handleCancel() {
  router.push('/');
}
</script>

<style scoped>
.add-palette-view {
  min-height: 100vh;
  background-color: var(--dark);
  padding: 2rem 0;
  overflow-y: auto;
}
</style>

<template>
  <div class="add-printer-view ">
    <div class="max-w-2xl mx-auto p-8">
      <div class="mb-8 text-center">
        <h1 class="text-3xl font-bold text-white mb-2">Add New Printer</h1>
        <p class="text-text-muted">Configure a new printer profile for your setup.</p>
      </div>

      <PrinterConfigForm :show-cancel="true" save-button-text="Save Printer" @save="handleSave"
        @cancel="handleCancel" />
    </div>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router';
import { usePrinterProfileStore } from '../stores/printerProfile';
import PrinterConfigForm from '../components/onboarding/PrinterConfigForm.vue';
import { v4 as uuidv4 } from 'uuid';

const router = useRouter();
const profileStore = usePrinterProfileStore();

async function handleSave(printerData) {
  try {
    if (!printerData.id) {
      printerData.id = uuidv4();
    }
    await profileStore.saveProfile(printerData);
    router.push('/');
  } catch (e) {
    console.error('Failed to save printer:', e);
    alert('Failed to save printer: ' + e.message);
  }
}

function handleCancel() {
  router.push('/');
}
</script>

<style scoped>
.add-printer-view {
  min-height: 100vh;
  background-color: var(--dark);
  padding: 2rem 0;
}
</style>

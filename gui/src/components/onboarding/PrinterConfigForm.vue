<template>
  <div class="printer-config-form">
    <PrinterProfileForm v-model="localData" />

    <div class="mt-8 flex justify-center gap-3">
      <button v-if="showCancel" @click="$emit('cancel')"
        class="px-6 py-2 bg-white/10 hover:bg-white/20 rounded transition-colors">
        Cancel
      </button>
      <button @click="handleSave" :disabled="!isValid"
        class="px-8 py-3 rounded-full text-lg shadow-lg hover:shadow-primary/50 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
        :class="showCancel ? 'btn-primary' : 'btn-cmyk'">
        {{ saveButtonText }}
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, computed } from 'vue';
import { v4 as uuidv4 } from 'uuid';
import PrinterProfileForm from '../PrinterProfileForm.vue';

const props = defineProps({
  initialData: {
    type: Object,
    default: () => ({
      id: uuidv4(),
      name: '',
      description: '',
      bed_width: 220,
      bed_depth: 220,
      nozzle_diameter: 0.4,
      min_layer_height: 0.1
    })
  },
  showCancel: {
    type: Boolean,
    default: false
  },
  saveButtonText: {
    type: String,
    default: 'Continue to Calibration'
  }
});

const emit = defineEmits(['save', 'cancel']);

const localData = ref({ ...props.initialData });

// Ensure ID exists
if (!localData.value.id) {
  localData.value.id = uuidv4();
}

const isValid = computed(() => {
  return localData.value.name && localData.value.name.trim().length > 0;
});

function handleSave() {
  if (!isValid.value) return;
  emit('save', { ...localData.value });
}

// Watch for external changes
watch(() => props.initialData, (newData) => {
  localData.value = { ...newData };
  if (!localData.value.id) {
    localData.value.id = uuidv4();
  }
}, { deep: true });
</script>

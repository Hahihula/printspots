<template>
  <div class="printer-profile-form-wrapper">
    <div class="printer-profile-form">
      <div class="form-group">
        <label class="form-label">Profile Name</label>
        <input v-model="form.name" type="text" class="form-input" placeholder="e.g. My Ender 3 V2" required />
      </div>

      <div class="form-group">
        <label class="form-label">Description (Optional)</label>
        <textarea v-model="form.description" class="form-input h-24 resize-none"
          placeholder="Any notes about this printer setup..."></textarea>
      </div>

      <div class="grid grid-cols-2 gap-6">
        <div class="form-group relative">
          <label class="form-label">Bed Width (mm)</label>
          <input v-model.number="form.bed_width" type="number" class="form-input" min="0" @input="onWidthChange" />
        </div>
        <div class="form-group relative">
          <div class="flex justify-between items-center mb-1">
            <label class="form-label">Bed Depth (mm)</label>
            <button @click="toggleLock" class="text-xs text-secondary hover:text-white flex items-center gap-1"
              :class="{ 'text-primary': isLocked }">
              <span v-if="isLocked">🔒 Linked</span>
              <span v-else>🔓 Unlinked</span>
            </button>
          </div>
          <input v-model.number="form.bed_depth" type="number" class="form-input" min="0" :disabled="isLocked" />
        </div>
      </div>

      <div class="grid grid-cols-2 gap-6">
        <div class="form-group">
          <label class="form-label">Nozzle Diameter (mm)</label>
          <input v-model.number="form.nozzle_diameter" type="number" step="0.1" class="form-input" />
        </div>
        <div class="form-group">
          <label class="form-label">Min Layer Height (mm)</label>
          <input v-model.number="form.min_layer_height" type="number" step="0.01" class="form-input" />
        </div>
      </div>

      <div class="form-group mt-4">
        <label class="flex items-center gap-3 cursor-pointer group">
          <div class="relative flex items-center">
            <input v-model="form.has_automatic_filament_change" type="checkbox" class="sr-only peer" />
            <div class="w-11 h-6 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary"></div>
          </div>
          <span class="text-sm font-medium text-text-muted group-hover:text-white transition-colors">My printer has automatic filament change (AMS, MMU, etc.)</span>
        </label>
      </div>
    </div>
  </div>
</template>

<script setup>
import { reactive, watch, ref } from 'vue';

const props = defineProps({
  initialData: {
    type: Object,
    default: () => ({
      name: '',
      description: '',
      bed_width: 220,
      bed_depth: 220,
      nozzle_diameter: 0.4,
      min_layer_height: 0.08,
      has_automatic_filament_change: false
    })
  }
});

const emit = defineEmits(['update:modelValue']);

const form = reactive({ ...props.initialData });
const isLocked = ref(true); // Default to locked

function toggleLock() {
  isLocked.value = !isLocked.value;
  if (isLocked.value) {
    form.bed_depth = form.bed_width;
  }
}

function onWidthChange() {
  if (isLocked.value) {
    form.bed_depth = form.bed_width;
  }
}

watch(form, (newVal) => {
  emit('update:modelValue', newVal);
}, { deep: true });
</script>

<style scoped>
.printer-profile-form-wrapper {
  display: flex;
  justify-content: center;
  width: 100%;
}

.printer-profile-form {
  max-width: 600px;
  width: 100%;
  padding: 0 1rem;
}
</style>

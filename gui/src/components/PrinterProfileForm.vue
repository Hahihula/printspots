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
      min_layer_height: 0.08
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

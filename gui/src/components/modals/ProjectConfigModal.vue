<template>
  <div v-if="show" class="modal-overlay" @click.self="handleCancel">
    <div class="modal-content">
      <h2 class="modal-title">New Project Configuration</h2>
      <p class="modal-subtitle">Configure your project settings</p>

      <form @submit.prevent="handleSubmit">
        <!-- Project Name -->
        <div class="form-group">
          <label class="form-label">Project Name</label>
          <input v-model="formData.name" type="text" required class="form-input" placeholder="My Project" />
        </div>

        <!-- Image Size -->
        <div class="form-group">
          <label class="form-label">Image Size (mm)</label>
          <p class="text-xs text-text-muted mb-2">Width/height of the final 3D object</p>
          <input v-model.number="formData.image_size_mm" type="number" step="0.1" min="1" required class="form-input"
            placeholder="100" />
        </div>

        <!-- Base Thickness -->
        <div class="form-group">
          <label class="form-label">Base Thickness (mm)</label>
          <p class="text-xs text-text-muted mb-2">Thickness of the base layer</p>
          <input v-model.number="formData.base_thickness" type="number" step="0.1" min="0.1" required class="form-input"
            placeholder="1.0" />
        </div>

        <!-- Layer Thickness -->
        <div class="form-group">
          <label class="form-label">Layer Thickness (mm)</label>
          <p class="text-xs text-text-muted mb-2">Layer height for printing (must match slicer settings)</p>
          <input v-model.number="formData.layer_thickness" type="number" step="0.01" min="0.01" required
            class="form-input" placeholder="0.05" />
        </div>

        <!-- Advanced Options -->
        <div class="form-group">
          <label class="checkbox-container">
            <input type="checkbox" v-model="formData.add_pads" />
            <span class="checkbox-label">Add Slicer Pads</span>
            <div class="tooltip-container">
              <span class="info-icon">?</span>
              <div class="tooltip-text">
                Adds a small square on the build plate for each color. This acts as a hack for slicers, allowing you to choose to print colors one-by-one without the slicer automatically re-centering the object. Highly recommended for manual filament changes.
              </div>
            </div>
          </label>
        </div>

        <div class="form-group">
          <label class="checkbox-container">
            <input type="checkbox" v-model="formData.flat_top" />
            <span class="checkbox-label">Flat Top Surface</span>
            <div class="tooltip-container">
              <span class="info-icon">?</span>
              <div class="tooltip-text">
                Expands the black background so that area under white parts is filled, resulting in a flat top surface. Warning: This requires many more filament changes and takes longer to print.
              </div>
            </div>
          </label>
          
          <div v-if="formData.flat_top && !printerStore.activeProfile?.has_automatic_filament_change" class="warning-box">
            <span class="warning-icon">⚠️</span>
            <p>Your printer is set to <strong>manual filament change</strong>. Enabling "Flat Top Surface" will require <strong>one manual change per color per layer</strong>, which can be extremely tedious!</p>
          </div>
        </div>

        <!-- Buttons -->
        <div class="modal-actions">
          <button type="button" @click="handleCancel" class="btn-cancel">
            Cancel
          </button>
          <button type="submit" class="btn-create">
            Create Project
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';
import { usePrinterProfileStore } from '../../stores/printerProfile';

const printerStore = usePrinterProfileStore();

const props = defineProps({
  show: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['save', 'cancel']);

const formData = ref({
  name: '',
  image_size_mm: 100,
  base_thickness: 1.0,
  layer_thickness: 0.05,
  add_pads: false,
  flat_top: false
});

// Reset form when modal is shown
watch(() => props.show, (newVal) => {
  if (newVal) {
    const today = new Date().toISOString().split('T')[0];
    formData.value = {
      name: `New Project ${today}`,
      image_size_mm: 100,
      base_thickness: 1.0,
      layer_thickness: 0.05,
      add_pads: false,
      flat_top: false
    };
  }
});

function handleSubmit() {
  emit('save', { ...formData.value });
}

function handleCancel() {
  emit('cancel');
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.75);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.modal-content {
  background: var(--dark-lighter);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 2rem;
  padding-right: 4rem;
  max-width: 500px;
  width: 90%;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
}

.modal-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: white;
  margin-bottom: 0.5rem;
}

.modal-subtitle {
  color: var(--text-muted);
  font-size: 0.875rem;
  margin-bottom: 1.5rem;
}

.form-group {
  margin-bottom: 1.25rem;
}

.form-label {
  display: block;
  color: var(--text-muted);
  font-size: 0.875rem;
  margin-bottom: 0.5rem;
  font-weight: 500;
}

.form-input,
.form-select {
  width: 100%;
  padding: 0.75rem 1rem;
  background: var(--dark);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: white;
  font-size: 1rem;
  transition: border-color 0.2s;
}

.form-input:focus,
.form-select:focus {
  outline: none;
  border-color: var(--primary);
}

.form-input::placeholder {
  color: var(--text-muted);
}

.modal-actions {
  display: flex;
  gap: 0.75rem;
  justify-content: flex-end;
  margin-top: 1.5rem;
}

.btn-cancel,
.btn-create {
  padding: 0.625rem 1.5rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-cancel {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: white;
}

.btn-cancel:hover {
  background: rgba(255, 255, 255, 0.1);
}

.btn-create {
  background: var(--primary);
  border: none;
  color: white;
}

.btn-create:hover:not(:disabled) {
  background: var(--primary-hover, #0097A7);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 188, 212, 0.3);
}

.btn-create:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.checkbox-container {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  cursor: pointer;
  user-select: none;
}

.checkbox-label {
  color: white;
  font-size: 0.875rem;
  font-weight: 500;
}

.tooltip-container {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.info-icon {
  width: 16px;
  height: 16px;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 10px;
  color: var(--text-muted);
  font-weight: bold;
}

.tooltip-text {
  visibility: hidden;
  position: absolute;
  bottom: 125%;
  left: 50%;
  transform: translateX(-50%);
  background-color: #333;
  color: #fff;
  text-align: center;
  padding: 8px 12px;
  border-radius: 6px;
  width: 250px;
  font-size: 12px;
  line-height: 1.4;
  z-index: 10;
  opacity: 0;
  transition: opacity 0.3s;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
  pointer-events: none;
}

.tooltip-container:hover .tooltip-text {
  visibility: visible;
  opacity: 1;
}

.warning-box {
  margin-top: 0.75rem;
  background: rgba(255, 152, 0, 0.1);
  border: 1px solid rgba(255, 152, 0, 0.2);
  border-radius: 8px;
  padding: 0.75rem;
  display: flex;
  gap: 0.75rem;
  align-items: flex-start;
}

.warning-icon {
  font-size: 1.25rem;
}

.warning-box p {
  margin: 0;
  font-size: 0.75rem;
  color: #ffb74d;
  line-height: 1.4;
}
</style>

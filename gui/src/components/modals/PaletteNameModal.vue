<template>
  <div v-if="show" class="modal-overlay" @click.self="$emit('cancel')">
    <div class="modal-content">
      <h2 class="modal-title">Name Your Palette</h2>
      <p class="modal-subtitle">Choose a descriptive name for your color palette</p>

      <input v-model="paletteName" type="text" class="modal-input" placeholder="e.g. My Default Palette"
        @keyup.enter="handleSave" ref="nameInput" />

      <div class="modal-actions">
        <button @click="$emit('cancel')" class="btn-cancel">
          Cancel
        </button>
        <button @click="handleSave" :disabled="!paletteName.trim()" class="btn-save">
          Save Palette
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, nextTick } from 'vue';

const props = defineProps({
  show: {
    type: Boolean,
    default: false
  },
  defaultName: {
    type: String,
    default: 'My Default Palette'
  }
});

const emit = defineEmits(['save', 'cancel']);

const paletteName = ref(props.defaultName);
const nameInput = ref(null);

watch(() => props.show, (newVal) => {
  if (newVal) {
    paletteName.value = props.defaultName;
    nextTick(() => {
      nameInput.value?.focus();
      nameInput.value?.select();
    });
  }
});

function handleSave() {
  if (paletteName.value.trim()) {
    emit('save', paletteName.value.trim());
  }
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
  max-width: 450px;
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

.modal-input {
  width: 100%;
  padding: 0.75rem 1rem;
  background: var(--dark);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: white;
  font-size: 1rem;
  margin-bottom: 1.5rem;
  transition: border-color 0.2s;
}

.modal-input:focus {
  outline: none;
  border-color: var(--primary);
}

.modal-input::placeholder {
  color: var(--text-muted);
}

.modal-actions {
  display: flex;
  gap: 0.75rem;
  justify-content: flex-end;
}

.btn-cancel,
.btn-save {
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

.btn-save {
  background: var(--primary);
  border: none;
  color: white;
}

.btn-save:hover:not(:disabled) {
  background: var(--primary-hover, #45a049);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(var(--primary-rgb), 0.3);
}

.btn-save:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>

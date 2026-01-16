<template>
  <div class="top-bar h-12 bg-dark-lighter border-b border-white/5 px-4 flex items-center justify-between">
    <div class="flex items-center gap-4">
      <Logo :size="32" />
      <h1 class="text-lg font-bold bg-gradient-to-r from-primary to-secondary bg-clip-text">
        PrintSpots
      </h1>
    </div>

    <div class="flex items-center gap-4">
      <!-- Printer Profile Selector -->
      <div class="flex items-center gap-2">
        <label class="text-xs text-text-muted font-bold">PRINTER</label>
        <select :value="profileStore.activeProfileId" @change="handleProfileChange"
          class="bg-dark border border-white/10 rounded px-3 py-1.5 text-sm min-w-[150px] focus:border-primary outline-none transition-colors">
          <option v-for="profile in profileStore.profiles" :key="profile.id" :value="profile.id">
            {{ profile.name }}
          </option>
          <option v-if="profileStore.profiles.length === 0" disabled>No Profiles</option>
          <option value="__add_new__" class="text-primary font-bold">+ Add New Printer...</option>
        </select>
      </div>

      <!-- Palette Selector -->
      <div class="flex items-center gap-2">
        <label class="text-xs text-text-muted font-bold">PALETTE</label>
        <select :value="paletteStore.activePaletteId" @change="handlePaletteChange"
          class="bg-dark border border-white/10 rounded px-3 py-1.5 text-sm min-w-[150px] focus:border-secondary outline-none transition-colors">
          <option v-for="p in paletteStore.palettes" :key="p.id" :value="p.id">
            {{ getPaletteName(p) }}
          </option>
          <option v-if="paletteStore.palettes.length === 0" disabled>No Palettes</option>
          <option value="__add_new__" class="text-secondary font-bold">+ Add New Palette...</option>
        </select>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center gap-2 ml-4">
        <button @click="$emit('new-project')"
          class="px-3 py-1.5 bg-primary text-white rounded text-sm hover:bg-primary/90 transition-colors font-bold">
          + New
        </button>
        <button @click="$emit('open-project')"
          class="px-3 py-1.5 bg-white/10 hover:bg-white/20 rounded text-sm transition-colors">
          Open
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { usePrinterProfileStore } from '../../stores/printerProfile';
import { usePaletteStore } from '../../stores/palette';
import { useRouter } from 'vue-router';
import Logo from '../Logo.vue';

const profileStore = usePrinterProfileStore();
const paletteStore = usePaletteStore();
const router = useRouter();

defineEmits(['new-project', 'open-project']);

function getPaletteName(palette) {
  return palette.name || `Palette ${palette.id.substring(0, 8)}`;
}

function handleProfileChange(event) {
  const value = event.target.value;
  if (value === '__add_new__') {
    // Navigate to add printer view
    router.push('/add-printer');
    // Reset select to current value
    event.target.value = profileStore.activeProfileId;
  } else {
    profileStore.setActiveProfile(value);
  }
}

function handlePaletteChange(event) {
  const value = event.target.value;
  if (value === '__add_new__') {
    // Navigate to add palette view
    router.push('/add-palette');
    // Reset select to current value
    event.target.value = paletteStore.activePaletteId;
  } else {
    paletteStore.setActivePalette(value);
  }
}
</script>

<style scoped>
.top-bar {
  height: 60px;
  background: var(--dark-lighter);
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

select {
  background: #f5f5f5;
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: #1a1a1a;
  padding: 0.375rem 0.75rem;
  border-radius: 6px;
  font-size: 0.875rem;
  min-width: 150px;
  transition: all 0.2s;
  cursor: pointer;
  color-scheme: light; /* Force light mode for dropdown */
}

select:hover {
  border-color: rgba(255, 255, 255, 0.2);
  background: rgba(255, 255, 255, 0.05);
}

select:focus {
  outline: none;
  border-color: var(--primary);
  box-shadow: 0 0 0 2px rgba(0, 188, 212, 0.1);
}

select option {
  background: #f5f5f5;
  color: #1a1a1a;
  padding: 0.5rem;
}

select option[value="__add_new__"] {
  color: #00BCD4;
  font-weight: 600;
  background: #f5f5f5;
}

select option:disabled {
  color: var(--text-muted);
}

button {
  transition: all 0.2s;
  font-weight: 600;
}

button:hover {
  transform: translateY(-1px);
}

button:active {
  transform: translateY(0);
}

.bg-primary:hover {
  background: var(--primary-dark) !important;
  box-shadow: 0 4px 12px rgba(0, 188, 212, 0.3);
}

.bg-white\/10:hover {
  background: rgba(255, 255, 255, 0.15) !important;
}

label {
  color: var(--text-muted);
  font-weight: 700;
  font-size: 0.75rem;
  letter-spacing: 0.05em;
}

/* Ensure logo is visible */
:deep(.logo-icon) {
  opacity: 1;
}

:deep(.logo-layer) {
  opacity: 1 !important;
}
</style>

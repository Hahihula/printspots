import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export const usePaletteStore = defineStore('palette', () => {
    const palettes = ref([]);
    const activePaletteId = ref(null);
    const initialized = ref(false);

    const activePalette = computed(() => {
        if (!activePaletteId.value) return null;
        return palettes.value.find(p => p.id === activePaletteId.value) || null;
    });

    const hasPalettes = computed(() => palettes.value.length > 0);

    async function init() {
        if (initialized.value) return;
        await loadPalettes();
        initialized.value = true;
    }

    async function loadPalettes() {
        try {
            const loaded = await invoke('get_palettes');
            palettes.value = loaded;
            
            if (!activePaletteId.value && loaded.length > 0) {
                activePaletteId.value = loaded[0].id;
            }
        } catch (e) {
            console.error("Failed to load palettes", e);
            palettes.value = [];
        }
    }

    function setActivePalette(id) {
        activePaletteId.value = id;
    }

    return {
        palettes,
        activePaletteId,
        activePalette,
        hasPalettes,
        initialized,
        init,
        loadPalettes,
        setActivePalette
    };
});

import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';

export const usePrinterProfileStore = defineStore('printerProfile', {
  state: () => ({
    profiles: [],
    activeProfileId: null,
    initialized: false,
  }),
  
  getters: {
    activeProfile(state) {
      return state.profiles.find(p => p.id === state.activeProfileId);
    },
    hasProfiles(state) {
      return state.profiles.length > 0;
    }
  },

  actions: {
    async init() {
        if (this.initialized) return;
        await this.loadProfiles();
        this.initialized = true;
    },

    async loadProfiles() {
      try {
        console.log("Loading profiles...");
        const profiles = await invoke('get_printer_profiles');
        this.profiles = profiles || [];
        
        // Set active profile if none selected but profiles exist
        if (!this.activeProfileId && this.profiles.length > 0) {
            this.activeProfileId = this.profiles[0].id;
        }
      } catch (e) {
        console.error('Failed to load printer profiles:', e);
        this.profiles = [];
      }
    },

    async saveProfile(profile) {
      try {
        await invoke('save_printer_profile', { profile });
        // Reload to ensure sync
        await this.loadProfiles();
        this.activeProfileId = profile.id;
      } catch (e) {
        console.error('Failed to save printer profile:', e);
        throw e;
      }
    },
    
    setActiveProfile(id) {
      this.activeProfileId = id;
    }
  }
});

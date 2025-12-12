import { createRouter, createWebHistory } from "vue-router";
import { usePrinterProfileStore } from './stores/printerProfile';
import OnboardingWizard from "./views/OnboardingWizard.vue";
import ProjectDashboard from "./views/ProjectDashboard.vue";
import AddPrinter from "./views/AddPrinter.vue";
import AddPalette from "./views/AddPalette.vue";

const routes = [
  { path: "/", component: ProjectDashboard },
  { path: "/onboarding", component: OnboardingWizard },
  { path: "/add-printer", component: AddPrinter },
  { path: "/add-palette", component: AddPalette },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach(async (to, from, next) => {
  // Allow onboarding route
  if (to.path === '/onboarding') {
    next();
    return;
  }

  const profileStore = usePrinterProfileStore();
  
  if (!profileStore.initialized) {
      await profileStore.init();
  }

  if (to.path !== '/onboarding') {
    try {
        const invoke = (await import('@tauri-apps/api/core')).invoke;
        const progress = await invoke('load_wizard_progress');
        if (progress) {
             next('/onboarding');
             return;
        }
    } catch(e) {
        console.error("Failed to check wizard progress", e);
    }
  }

  if (!profileStore.hasProfiles) {
    next('/onboarding');
  } else {
    next(); 
  }
});

export default router;

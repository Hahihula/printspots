<template>
  <div class="app-container h-screen w-screen bg-dark text-white relative">

    <!-- Global Splash Screen -->
    <transition name="fade">
      <div v-if="loading" class="splash-screen">
        <Logo :size="120" style="margin-bottom: 2rem;" />
        <h1 class="splash-title">
          PrintSpots
        </h1>
        <p class="splash-subtitle">Initializing...</p>
      </div>
    </transition>

    <router-view v-if="!loading" />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { usePrinterProfileStore } from './stores/printerProfile';
import Logo from './components/Logo.vue';

const router = useRouter();
const profileStore = usePrinterProfileStore();
const loading = ref(true);

onMounted(async () => {
  const startTime = Date.now();

  // Initialize Store (loads profiles)
  await profileStore.init();

  // Minimum splash duration 1.5s
  const elapsed = Date.now() - startTime;
  const remaining = Math.max(0, 1500 - elapsed);

  setTimeout(() => {
    loading.value = false;
    if (!profileStore.hasProfiles) {
      router.push('/onboarding');
    } else if (router.currentRoute.value.path === '/') {
      router.push('/'); // Dashboard
    }
  }, remaining);
});
</script>

<style>
html,
body,
#app {
  height: 100%;
  margin: 0;
  padding: 0;
  overflow: hidden;
  /* Prevent body scroll */
}

/* Splash Screen Styles */
.splash-screen {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background-color: var(--dark);
  color: white;
}

.splash-title {
  font-size: 3rem;
  font-weight: bold;
  margin-bottom: 1rem;
  background: linear-gradient(90deg, var(--primary), var(--secondary), var(--accent));
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  text-align: center;
}

.splash-subtitle {
  color: var(--text-muted);
  font-size: 1.25rem;
  letter-spacing: 0.05em;
  animation: pulse 2s infinite;
}

@keyframes pulse {

  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0.5;
  }
}
</style>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
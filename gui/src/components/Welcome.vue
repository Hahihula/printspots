<template>
  <div class="splash-screen">
    <div class="logo-icon">
      <div class="logo-layer"></div>
      <div class="logo-layer"></div>
      <div class="logo-layer"></div>
      <div class="logo-layer"></div>
    </div>
    <span class="logo-text">PrintSpots</span>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router';
import { onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const router = useRouter();

onMounted(async () => {
  const minDisplayTime = 500; // 0.5 seconds
  const startTime = Date.now();

  let configExists = false;
  let palettesExist = 0;

  try {
    configExists = await invoke('check_config_exists');
    const palettesCount = await invoke('check_pallettes_exists');
    palettesExist = palettesCount;
  } catch (e) {
    console.error('Error checking config or palettes:', e);
    // Decide on a fallback, e.g., navigate to configure if an error occurs
    router.push('/configure');
    return;
  }

  const elapsedTime = Date.now() - startTime;
  const remainingTime = minDisplayTime - elapsedTime;

  if (remainingTime > 0) {
    setTimeout(() => {
      navigateBasedOnConfig(configExists, palettesExist);
    }, remainingTime);
  } else {
    navigateBasedOnConfig(configExists, palettesExist);
  }
});

const navigateBasedOnConfig = (configExists, palettesExist) => {
  if (!configExists) {
    router.push('/configure');
  } else if (palettesExist === 0) {
    router.push('/calibration');
  } else {
    router.push('/generation');
  }
};
</script>

<style scoped>
.splash-screen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: var(--dark); /* Using project's dark theme color */
  display: flex;
  flex-direction: column; /* Stack logo and text vertically */
  justify-content: center;
  align-items: center;
  gap: 1rem; /* Space between logo icon and text */
  z-index: 9999; /* Ensure it covers everything */
}

.logo-icon {
  width: 80px; /* Increased size for splash screen */
  height: 80px; /* Increased size for splash screen */
  position: relative;
}

.logo-layer {
  position: absolute;
  width: 100%;
  height: 16px; /* Increased size */
  border-radius: 4px; /* Increased size */
  animation: float 3s ease-in-out infinite;
}

.logo-layer:nth-child(1) {
  background: #00bcd4;
  top: 0;
  animation-delay: 0s;
}

.logo-layer:nth-child(2) {
  background: #e91e63;
  top: 24px; /* Adjusted position */
  animation-delay: 0.2s;
}

.logo-layer:nth-child(3) {
  background: #ffeb3b;
  top: 48px; /* Adjusted position */
  animation-delay: 0.4s;
}

.logo-layer:nth-child(4) {
  background: #000;
  top: 70px; /* Adjusted position */
  animation-delay: 0.4s;
}

@keyframes float {
  0%, to {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-6px); /* Increased float distance */
  }
}

.logo-text {
  font-size: 2.5rem; /* Increased size for splash screen */
  font-weight: 700;
  color: var(--primary); /* Assuming --primary is defined globally or in printspots.css */
  margin-top: 1rem; /* Space between icon and text */
}
</style>
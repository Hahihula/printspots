<template>
  <div class="logo-icon" :style="{ width: size + 'px', height: size + 'px' }">
    <div class="logo-layer"></div>
    <div class="logo-layer"></div>
    <div class="logo-layer"></div>
    <div class="logo-layer"></div>
  </div>
</template>

<script setup>
defineProps({
  size: {
    type: Number,
    default: 80
  }
});
</script>

<style scoped>
.logo-icon {
  position: relative;
  display: block;
  /* Ensure it holds space */
  /* background: rgba(255,255,255,0.05); Debug background */
}

.logo-layer {
  position: absolute;
  left: 0;
  width: 100%;
  height: 20%;
  /* Proportional height */
  border-radius: 4px;
  animation: float 3s ease-in-out infinite;
  display: block;
}

.logo-layer:nth-child(1) {
  background: #00bcd4;
  top: 0;
  animation-delay: 0s;
}

.logo-layer:nth-child(2) {
  background: #e91e63;
  top: 33%;
  animation-delay: 0.2s;
}

.logo-layer:nth-child(3) {
  background: #ffeb3b;
  top: 66%;
  animation-delay: 0.4s;
}

.logo-layer:nth-child(4) {
  background: #000;
  top: 100%;
  /* Just below */
  margin-top: -10%;
  /* Slight overlap */
  animation-delay: 0.4s;
}

@keyframes float {

  0%,
  to {
    transform: translateY(0);
  }

  50% {
    transform: translateY(-10%);
  }
}
</style>

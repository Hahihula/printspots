<template>
  <div class="split-layout flex w-full h-full overflow-hidden">
    <!-- Left Panel: Editor -->
    <div class="left-panel flex-1 bg-dark relative border-r border-white/10">
      <slot name="left"></slot>
    </div>

    <!-- Right Panel: Prediction -->
    <div class="right-panel flex-1 bg-dark relative">
      <slot name="right"></slot>
    </div>
  </div>
</template>

<style scoped>
/* Future: Add resize handle logic here if needed */
</style>

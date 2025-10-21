<template>
  <header class="header">
    <div class="nav-container">
      <a href="/" class="logo">
        <div class="logo-icon">
          <div class="logo-layer"></div>
          <div class="logo-layer"></div>
          <div class="logo-layer"></div>
          <div class="logo-layer"></div>
        </div>
        PrintSpots
      </a>
      <ul class="nav-links">
        <li><router-link to="/">Welcome</router-link></li>
        <li><router-link to="/configure">Configure</router-link></li>
        <li><router-link to="/calibration">Calibration</router-link></li>
        <li><router-link to="/generation">Generation</router-link></li>
      </ul>
    </div>
  </header>
  <main>
    <router-view />
  </main>
</template>

<style>
/* Styles will be added here later */
</style>
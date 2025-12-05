
import { createRouter, createWebHistory } from 'vue-router';
import Welcome from './components/Welcome.vue';
import Configure from './components/Configure.vue';
import Calibration from './components/Calibration.vue';
import Generation from './components/Generation.vue';

const routes = [
  { path: '/', component: Welcome },
  { path: '/configure', component: Configure },
  { path: '/calibration', component: Calibration },
  { path: '/generation', component: Generation },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

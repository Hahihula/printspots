<template>
  <div class="h-screen flex flex-col bg-dark text-white overflow-hidden">
    <!-- Top Navigation Bar -->
    <TopBar @new-project="handleNewProject" @open-project="handleOpenProject" />

    <!-- Main Workspace -->
    <main class="flex-1 relative overflow-hidden">

      <!-- Empty State -->
      <ProjectEmptyState v-if="!projectStore.currentProject" @new-project="handleNewProject"
        @open-project="handleOpenProject" />

      <!-- Project Editor View -->
      <ProjectEditor v-else :project="projectStore.currentProject" />

    </main>

    <!-- Project Config Modal -->
    <ProjectConfigModal :show="projectStore.showConfigModal" @save="handleSaveConfig" @cancel="handleCancelConfig" />
  </div>
</template>

<script setup>
import { onMounted } from 'vue';
import TopBar from '../components/dashboard/TopBar.vue';
import ProjectEmptyState from '../components/dashboard/ProjectEmptyState.vue';
import ProjectEditor from '../components/editor/ProjectEditor.vue';
import ProjectConfigModal from '../components/modals/ProjectConfigModal.vue';
import { usePrinterProfileStore } from '../stores/printerProfile';
import { usePaletteStore } from '../stores/palette';
import { useProjectStore } from '../stores/project';

const profileStore = usePrinterProfileStore();
const paletteStore = usePaletteStore();
const projectStore = useProjectStore();

onMounted(async () => {
  // Ensure all global data is loaded
  await Promise.all([
    profileStore.init(),
    paletteStore.init()
  ]);
});

function handleNewProject() {
  // Validate we have necessary printer/palette info?
  if (!profileStore.hasProfiles || !paletteStore.hasPalettes) {
    alert("Please ensure you have at least one Printer Profile and one Palette created.");
    return;
  }
  projectStore.createNewProject();
}

async function handleSaveConfig(config) {
  try {
    await projectStore.confirmProjectConfig(config);
  } catch (err) {
    alert(`Failed to create project: ${err.message}`);
  }
}

function handleCancelConfig() {
  projectStore.cancelProjectConfig();
}

async function handleOpenProject() {
  try {
    await projectStore.loadProjectFromFile();
  } catch (err) {
    alert(`Failed to open project: ${err.message}`);
  }
}

function closeProject() {
  projectStore.closeProject();
}
</script>

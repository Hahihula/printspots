<template>
  <div class="h-full w-full relative bg-dark" ref="editorContainer"></div>
</template>

<script setup>
import { onMounted, ref, watch, onBeforeUnmount } from 'vue';
import ImageEditor from 'tui-image-editor';
// Import TUI styles - adjust path if needed based on framework
import 'tui-image-editor/dist/tui-image-editor.css';
import 'tui-color-picker/dist/tui-color-picker.css';

const props = defineProps({
  imageUrl: {
    type: String,
    default: null
  }
});

const editorContainer = ref(null);
let editorInstance = null;

// Custom Dark Theme for TUI Image Editor
const darkTheme = {
  'common.bi.image': '', // No BI logo
  'common.bisize.width': '0px',
  'common.bisize.height': '0px',
  'common.backgroundColor': '#1E293B', // Dark background
  'header.display': 'none', // Hide standard header
  'menu.normalIcon.path': '', // Use defaults or override
  'menu.activeIcon.path': '',
  'menu.disabledIcon.path': '',
  'menu.hoverIcon.path': '',
  'submenu.backgroundColor': '#0F172A',
  'submenu.partition.color': '#334155',
  'submenu.normalIcon.path': '',
  'submenu.activeIcon.path': '',
  'submenu.iconSize.width': '32px',
  'submenu.iconSize.height': '32px',
  'submenu.normalLabel.color': '#94a3b8',
  'submenu.activeLabel.color': '#fff',
  'checkbox.border': '1px solid #475569',
  'checkbox.backgroundColor': '#1e293b',
  'range.pointer.color': '#00bcd4', // Primary Cyan
  'range.bar.color': '#475569',
  'range.subbar.color': '#475569',
  'range.value.color': '#fff',
  'range.value.fontWeight': 'lighter',
  'range.title.color': '#fff',
  'range.title.fontWeight': 'lighter',
  'colorpicker.button.border': '1px solid #475569',
  'colorpicker.title.color': '#fff',
};

onMounted(() => {
  initEditor();
});

watch(() => props.imageUrl, (newUrl) => {
  if (editorInstance && newUrl) {
    editorInstance.loadImageFromURL(newUrl, 'Source Image').then(() => {
      editorInstance.clearUndoStack();
    });
  }
});

// Resize observer to handle layout changes
let resizeObserver = null;

function initEditor() {
  if (!editorContainer.value) return;

  editorInstance = new ImageEditor(editorContainer.value, {
    includeUI: {
      loadImage: {
        path: props.imageUrl || '',
        name: 'Source Image'
      },
      theme: darkTheme,
      menu: ['crop', 'flip', 'rotate', 'filter'], // Limit to requested features initially
      initMenu: 'filter',
      uiSize: {
        width: '100%',
        height: '100%'
      },
      menuBarPosition: 'bottom'
    },
    cssMaxWidth: 700,
    cssMaxHeight: 500,
    selectionStyle: {
      cornerSize: 20,
      rotatingPointOffset: 70
    }
  });

  // Add resize observer to force refresh if layout changes (split screen resizing)
  resizeObserver = new ResizeObserver(() => {
    // TUI might need a resize call, generally it handles window resize but div resize might need help
    // editorInstance.ui.resizeEditor(); // Internal API, risky but often needed
  });
  resizeObserver.observe(editorContainer.value);
}

function getImageDataUrl() {
  if (!editorInstance) {
    throw new Error('Editor not initialized');
  }
  return editorInstance.toDataURL();
}

onBeforeUnmount(() => {
  if (resizeObserver) resizeObserver.disconnect();
  if (editorInstance) {
    editorInstance.destroy();
  }
});

defineExpose({
  getImageDataUrl
});
</script>

<style scoped>
/* Override TUI CSS to match app theme deeper if needed */
:deep(.tui-image-editor-container) {
  background-color: var(--dark-lighter) !important;
}

:deep(.tui-image-editor-main-container) {
  background-color: var(--dark) !important;
  border: none !important;
}

:deep(.tui-image-editor-header-buttons) {
  display: none;
}
</style>

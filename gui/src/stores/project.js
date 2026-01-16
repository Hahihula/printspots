import { defineStore } from 'pinia';
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

export const useProjectStore = defineStore('project', () => {
    const currentProject = ref(null);
    const isDirty = ref(false);
    const isGeneratingPrediction = ref(false);
    const isGenerating3MF = ref(false);
    const imageCanvasRef = ref(null);
    const showConfigModal = ref(false);

    function createNewProject() {
        // Show config modal instead of creating immediately
        showConfigModal.value = true;
    }

    async function confirmProjectConfig(config) {
        try {
            const projectId = crypto.randomUUID();
            const now = Date.now();

            // Create full project structure
            const projectData = {
                id: projectId,
                name: config.name,
                image_size_mm: config.image_size_mm,
                base_thickness: config.base_thickness,
                layer_thickness: config.layer_thickness,
                add_pads: config.add_pads,
                flat_top: config.flat_top,
                source_image: null,
                settings: {},
                last_modified: now
            };

            // Save full project data to backend
            await invoke('save_project_config', {
                projectData
            });

            // Create project in store
            currentProject.value = {
                ...projectData,
                sourceImage: null,
                sourceImageUrl: null,
                predictionImageUrl: null,
                meshStats: null,
                lastModified: now
            };

            isDirty.value = false;
            showConfigModal.value = false;
        } catch (err) {
            console.error('Failed to save project config:', err);
            throw err;
        }
    }

    function cancelProjectConfig() {
        showConfigModal.value = false;
    }

    function loadProject(projectData) {
        currentProject.value = projectData;
        isDirty.value = false;
    }

    function closeProject() {
        currentProject.value = null;
        isDirty.value = false;
    }

    async function loadSourceImage() {
        if (!currentProject.value) return;

        try {
            const selected = await open({
                multiple: false,
                filters: [{
                    name: 'Images',
                    extensions: ['png', 'jpg', 'jpeg', 'bmp', 'webp']
                }]
            });

            if (selected) {
                // Import the image to the project directory and get base64 data
                const imageDataUrl = await invoke('import_image', {
                    projectId: currentProject.value.id,
                    sourcePath: selected
                });

                currentProject.value.sourceImage = selected; // Keep original path for reference
                currentProject.value.sourceImageUrl = imageDataUrl; // Use base64 data URL
                isDirty.value = true;
            }
        } catch (err) {
            console.error('Failed to open image:', err);
        }
    }

    async function generatePrediction() {
        if (!currentProject.value || !imageCanvasRef.value) {
            console.error('No project or image canvas available');
            return;
        }

        try {
            isGeneratingPrediction.value = true;

            // Get stores
            const { usePaletteStore } = await import('./palette');
            const { usePrinterProfileStore } = await import('./printerProfile');
            const paletteStore = usePaletteStore();
            const profileStore = usePrinterProfileStore();

            if (!paletteStore.activePaletteId) {
                throw new Error('No palette selected');
            }
            if (!profileStore.activeProfileId) {
                throw new Error('No printer profile selected');
            }

            // Get edited image data from canvas
            const imageData = imageCanvasRef.value.getImageDataUrl();

            // Build project config
            const projectConfig = {
                image_size_mm: currentProject.value.image_size_mm,
                base_thickness: currentProject.value.base_thickness,
                layer_thickness: currentProject.value.layer_thickness,
            };

            // Call backend
            const predictionDataUrl = await invoke('generate_prediction', {
                projectId: currentProject.value.id,
                imageData,
                projectConfig: {
                    ...projectConfig,
                    add_pads: currentProject.value.add_pads,
                    flat_top: currentProject.value.flat_top
                },
                printerProfileId: profileStore.activeProfileId,
                paletteId: paletteStore.activePaletteId,
            });

            currentProject.value.predictionImageUrl = predictionDataUrl;
            isDirty.value = true;
        } catch (err) {
            console.error('Failed to generate prediction:', err);
            throw err;
        } finally {
            isGeneratingPrediction.value = false;
        }
    }

    async function generate3MF() {
        if (!currentProject.value) {
            throw new Error('No project available');
        }

        if (!currentProject.value.predictionImageUrl) {
            throw new Error('No prediction available. Generate prediction first.');
        }

        try {
            isGenerating3MF.value = true;

            // Get stores
            const { usePaletteStore } = await import('./palette');
            const { usePrinterProfileStore } = await import('./printerProfile');
            const paletteStore = usePaletteStore();
            const profileStore = usePrinterProfileStore();

            if (!paletteStore.activePaletteId) {
                throw new Error('No palette selected');
            }
            if (!profileStore.activeProfileId) {
                throw new Error('No printer profile selected');
            }

            // Build project config
            const projectConfig = {
                image_size_mm: currentProject.value.image_size_mm,
                base_thickness: currentProject.value.base_thickness,
                layer_thickness: currentProject.value.layer_thickness,
                add_pads: currentProject.value.add_pads,
                flat_top: currentProject.value.flat_top
            };

            // Call backend
            const meshStats = await invoke('generate_3mf', {
                projectId: currentProject.value.id,
                projectConfig,
                printerProfileId: profileStore.activeProfileId,
                paletteId: paletteStore.activePaletteId,
            });

            currentProject.value.meshStats = meshStats;
            isDirty.value = true;
            return meshStats;
        } catch (err) {
            console.error('Failed to generate 3MF:', err);
            throw err;
        } finally {
            isGenerating3MF.value = false;
        }
    }

    function setImageCanvasRef(ref) {
        imageCanvasRef.value = ref;
    }

    async function loadProjectFromFile() {
        try {
            // Open file dialog for project.json
            const selected = await open({
                multiple: false,
                filters: [{
                    name: 'Project Files',
                    extensions: ['json']
                }],
                title: 'Open Project'
            });

            if (!selected) return;

            // Load project data from backend
            const projectData = await invoke('load_project_from_file', {
                path: selected
            });

            // Get project directory
            const projectDir = selected.substring(0, selected.lastIndexOf('/'));

            // Try to load source image if it exists
            let sourceImageUrl = null;
            try {
                // Use read_project_image to get base64 data URL from existing file
                sourceImageUrl = await invoke('read_project_image', {
                    projectId: projectData.id,
                    imageName: 'source_image.png'
                });
            } catch (e) {
                console.log('No source image found:', e);
            }

            // Try to load prediction image if it exists
            let predictionImageUrl = null;
            try {
                // Read prediction as base64
                predictionImageUrl = await invoke('read_project_image', {
                    projectId: projectData.id,
                    imageName: 'prediction.png'
                });
            } catch (e) {
                console.log('No prediction found:', e);
            }

            // Create project in store
            currentProject.value = {
                id: projectData.id,
                name: projectData.name,
                sourceImage: projectData.source_image,
                sourceImageUrl,
                predictionImageUrl,
                meshStats: null, // Will be loaded if 3MF exists
                image_size_mm: projectData.image_size_mm,
                base_thickness: projectData.base_thickness,
                layer_thickness: projectData.layer_thickness,
                add_pads: projectData.add_pads,
                flat_top: projectData.flat_top,
                settings: projectData.settings || {},
                lastModified: projectData.last_modified
            };

            isDirty.value = false;
        } catch (err) {
            console.error('Failed to load project:', err);
            throw err;
        }
    }

    return {
        currentProject,
        isDirty,
        isGeneratingPrediction,
        isGenerating3MF,
        showConfigModal,
        createNewProject,
        confirmProjectConfig,
        cancelProjectConfig,
        loadProject,
        closeProject,
        loadSourceImage,
        generatePrediction,
        generate3MF,
        setImageCanvasRef,
        loadProjectFromFile
    };
});


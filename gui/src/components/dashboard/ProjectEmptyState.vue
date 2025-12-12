<template>
  <div class="empty-state">
    <div class="content-wrapper">
      <div class="header-section">
        <h1 class="title">Welcome to PrintSpots</h1>
        <p class="subtitle">Create lithophanes from your images with precision and ease</p>
      </div>

      <div class="actions-section">
        <button @click="$emit('new-project')" class="action-btn primary-btn">
          <svg xmlns="http://www.w3.org/2000/svg" class="icon" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          <span class="btn-text">New Project</span>
        </button>

        <button @click="$emit('open-project')" class="action-btn secondary-btn">
          <svg xmlns="http://www.w3.org/2000/svg" class="icon" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
          <span class="btn-text">Open Project</span>
        </button>
      </div>

      <div class="recent-projects-section">
        <h2 class="section-title">Recent Projects</h2>
        <p class="empty-message">No recent projects yet. Create your first project to get started!</p>
      </div>
    </div>
  </div>
</template>

<script setup>
defineEmits(['new-project', 'open-project']);
</script>

<style scoped>
.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  padding: 2rem;
  background: linear-gradient(135deg, rgba(var(--primary-rgb), 0.05) 0%, rgba(var(--secondary-rgb), 0.05) 100%);
}

.content-wrapper {
  max-width: 600px;
  width: 100%;
  text-align: center;
}

.header-section {
  margin-bottom: 3rem;
}

.title {
  font-size: 2.5rem;
  font-weight: 800;
  background: linear-gradient(135deg, var(--primary) 0%, var(--secondary) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  margin-bottom: 0.5rem;
}

.subtitle {
  font-size: 1.125rem;
  color: var(--text-muted);
}

.actions-section {
  display: flex;
  gap: 1rem;
  margin-bottom: 4rem;
}

.action-btn {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.75rem;
  padding: 2rem 1.5rem;
  border: 2px solid transparent;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.05);
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 1rem;
  font-weight: 600;
}

.action-btn:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
}

.primary-btn {
  border-color: var(--primary);
  color: var(--primary);
}

.primary-btn:hover {
  background: rgba(var(--primary-rgb), 0.1);
  box-shadow: 0 8px 24px rgba(var(--primary-rgb), 0.3);
}

.secondary-btn {
  border-color: var(--secondary);
  color: var(--secondary);
}

.secondary-btn:hover {
  background: rgba(var(--secondary-rgb), 0.1);
  box-shadow: 0 8px 24px rgba(var(--secondary-rgb), 0.3);
}

.icon {
  width: 3rem;
  height: 3rem;
}

.btn-text {
  font-size: 1.125rem;
}

.recent-projects-section {
  padding-top: 2rem;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.section-title {
  font-size: 1.25rem;
  font-weight: 600;
  margin-bottom: 1rem;
  color: white;
}

.empty-message {
  color: var(--text-muted);
  font-size: 0.875rem;
}
</style>

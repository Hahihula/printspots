<template>
  <button :class="buttonClass">
    <slot></slot>
  </button>
</template>

<script>
export default {
  name: 'PrintspotsButton',
  props: {
    variant: {
      type: String,
      default: 'primary',
      validator: (value) => ['primary', 'secondary'].includes(value)
    }
  },
  computed: {
    buttonClass() {
      return ['btn', `btn-${this.variant}`];
    }
  }
}
</script>

<style>
:root {
    --primary: #00BCD4;      /* Cyan */
    --primary-dark: #0097A7;  /* Darker Cyan */
    --secondary: #E91E63;     /* Magenta */
    --accent: #FFEB3B;        /* Yellow */
    --dark: #0A0E27;
    --dark-lighter: #151A3A;
    --text: #E0E6ED;
    --text-muted: #9CA3AF;
    --black-key: #212121;     /* Key/Black */
}

.btn {
    padding: 1rem 2rem;
    border-radius: 50px;
    text-decoration: none;
    font-weight: 600;
    transition: all 0.3s ease;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    border: 2px solid transparent;
    cursor: pointer;
}

.btn-primary {
    background: linear-gradient(135deg, var(--primary), var(--primary-dark));
    color: white;
    box-shadow: 0 10px 30px rgba(0, 212, 170, 0.3);
}

.btn-primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 15px 40px rgba(0, 212, 170, 0.4);
}

.btn-secondary {
    background: transparent;
    color: var(--text);
    border-color: var(--text-muted);
}

.btn-secondary:hover {
    border-color: var(--primary);
    color: var(--primary);
}
</style>
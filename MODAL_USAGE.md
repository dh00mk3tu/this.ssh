# Modal Component Usage Guide

## Overview

The `Modal.vue` component is a reusable, flexible modal system that uses Vue 3 slots to provide customizable content areas. It follows your app's color scheme and provides consistent behavior across all modals.

## Features

- **Slot-based architecture** for maximum flexibility
- **Consistent styling** using your app's CSS variables
- **Backdrop click handling** (configurable)
- **Responsive design** for mobile and desktop
- **Accessibility features** (ARIA labels, keyboard support)
- **Customizable header, content, and footer** areas

## Basic Usage

```vue
<template>
  <Modal :is-open="isModalOpen" @close="closeModal">
    <!-- Header Slot -->
    <template #header>
      <h2>My Modal Title</h2>
    </template>

    <!-- Content Slot -->
    <template #content>
      <p>Your modal content goes here...</p>
    </template>

    <!-- Footer Slot (optional) -->
    <template #footer>
      <button @click="closeModal">Close</button>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import Modal from "./components/Modal.vue";

const isModalOpen = ref(false);

const closeModal = () => {
  isModalOpen.value = false;
};
</script>
```

## Props

| Prop              | Type      | Default | Description                            |
| ----------------- | --------- | ------- | -------------------------------------- |
| `isOpen`          | `boolean` | `false` | Controls modal visibility              |
| `closeOnBackdrop` | `boolean` | `true`  | Whether clicking backdrop closes modal |

## Events

| Event   | Description                     |
| ------- | ------------------------------- |
| `close` | Emitted when modal should close |

## Slots

### Header Slot (`#header`)

- **Purpose**: Modal title and header content
- **Content**: Any HTML content (recommended: headings, titles)
- **Default**: Simple "Modal Title" text

### Content Slot (`#content`)

- **Purpose**: Main modal content
- **Content**: Any HTML content (forms, text, lists, etc.)
- **Default**: "Modal content goes here" text

### Footer Slot (`#footer`)

- **Purpose**: Action buttons and footer content
- **Content**: Any HTML content (recommended: buttons, actions)
- **Default**: Not rendered if not provided

## Examples

### Simple Information Modal

```vue
<Modal :is-open="showInfo" @close="showInfo = false">
  <template #header>
    <h2>Information</h2>
  </template>
  
  <template #content>
    <p>This is an informational message.</p>
  </template>
</Modal>
```

### Form Modal (like CreateKeyModal)

```vue
<Modal :is-open="showForm" @close="closeForm">
  <template #header>
    <h2>Create New Item</h2>
  </template>
  
  <template #content>
    <form @submit.prevent="submitForm">
      <input v-model="formData.name" placeholder="Name" />
      <textarea v-model="formData.description" placeholder="Description" />
    </form>
  </template>
  
  <template #footer>
    <button @click="closeForm">Cancel</button>
    <button @click="submitForm">Submit</button>
  </template>
</Modal>
```

### Confirmation Modal

```vue
<Modal :is-open="showConfirm" @close="showConfirm = false">
  <template #header>
    <h2>Confirm Action</h2>
  </template>
  
  <template #content>
    <p>Are you sure you want to delete this item?</p>
    <p class="text-sm text-red-500">This action cannot be undone.</p>
  </template>
  
  <template #footer>
    <button @click="showConfirm = false">Cancel</button>
    <button @click="confirmDelete" class="btn-danger">Delete</button>
  </template>
</Modal>
```

## Styling

The Modal component automatically uses your app's color variables:

- `--primary-bg`: Modal background
- `--primary-highlight`: Borders and highlights
- `--secondary-bg`: Secondary elements
- `--secondary-highlight`: Text and icons

## Responsive Behavior

- **Desktop**: Fixed width with max-width constraints
- **Mobile**: Full width with margin padding
- **Height**: Automatic with max-height and scrolling
- **Padding**: Responsive padding adjustments

## Best Practices

1. **Always provide a header** for accessibility
2. **Use semantic HTML** in your slots
3. **Keep content focused** - one modal, one purpose
4. **Provide clear actions** in the footer
5. **Handle loading states** appropriately
6. **Test on mobile devices** for responsive behavior

## Migration from Old Modal

If you're migrating from the old modal system:

1. **Replace modal wrapper** with `<Modal>` component
2. **Move header content** to `#header` slot
3. **Move main content** to `#content` slot
4. **Move action buttons** to `#footer` slot
5. **Remove custom modal styles** (use component's built-in styling)
6. **Update event handlers** to use `@close` event

## Troubleshooting

### Modal not showing

- Check `isOpen` prop is `true`
- Verify z-index isn't conflicting
- Ensure parent container has proper positioning

### Styling issues

- Verify CSS variables are defined
- Check for conflicting Tailwind classes
- Ensure proper CSS specificity

### Content not rendering

- Verify slot names are correct (`#header`, `#content`, `#footer`)
- Check for syntax errors in slot content
- Ensure Vue 3 composition API is properly set up

<template>
  <div :class="`status-dot mx-1 ${isActive ? 'bg-green-600' : 'bg-red-500'}`" />
</template>

<script setup lang="ts">
defineProps<{
  isActive: boolean;
}>();
</script>

<style scoped>
.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  animation: blinkAnimation 2s infinite ease-in-out;
}

@keyframes blinkAnimation {
  0% {
    filter: brightness(1);
  }
  50% {
    filter: brightness(0.5);
  }
  100% {
    filter: brightness(1);
  }
}
</style>

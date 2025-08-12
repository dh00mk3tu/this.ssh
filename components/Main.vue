<template>
  <div class="root-container">
    <Header :is-ssh-agent-running="isSSHAgentRunning" />
    <div class="main-container max-h-screen">
      <!-- <h1 class="text-xl font-bold mb-4">SSH Keys</h1> -->
      <Keys ref="keysComponent" />
    </div>

    <!-- Floating Action Button -->
    <FloatingActionButton @key-created="handleKeyCreated" />
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import Header from "./Header.vue";
import Keys from "./Keys.vue";
import FloatingActionButton from "./FloatingActionButton.vue";

const isSSHAgentRunning = ref<boolean>(false);
const keysComponent = ref<InstanceType<typeof Keys> | null>(null);

onMounted(async () => {
  isSSHAgentRunning.value = await invoke<boolean>("is_ssh_agent_running");
});

const handleKeyCreated = () => {
  // Refresh the keys list when a new key is created
  if (keysComponent.value) {
    keysComponent.value.fetchSSHKeys();
    keysComponent.value.getLoadedSSHAgentKeys();
  }
};
</script>

<styles scoped>

</styles>

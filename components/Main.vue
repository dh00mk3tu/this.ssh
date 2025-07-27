<template>
  <div class="root-container">
    <Header :is-ssh-agent-running="isSSHAgentRunning" />
    <div class="main-container max-h-screen">
      <!-- <h1 class="text-xl font-bold mb-4">SSH Keys</h1> -->
      <Keys />
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import Header from "./Header.vue";
import Keys from "./Keys.vue";

const isSSHAgentRunning = ref<boolean>(false);
onMounted(async () => {
  isSSHAgentRunning.value = await invoke<boolean>("is_ssh_agent_running");
});
</script>

<styles scoped>

</styles>

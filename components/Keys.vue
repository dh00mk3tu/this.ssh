<template>
  <div class="container">
    <div v-if="keysState.status === 'idle'" class="idle-state">
      <p class="text-center">{{ keysState.message }}</p>
      <p class="text-center">
        <button @click="fetchSSHKeys">Fetch SSH Keys</button>
      </p>
    </div>

    <div v-if="keysState.status === 'loading'" class="loading-state">
      <p class="text-center">{{ keysState.message }}</p>
    </div>

    <div v-if="keysState.status === 'failed'" class="error-state">
      <p class="text-center">Error: {{ keysState.data }}</p>
    </div>

    <div v-if="keysState.status === 'success'">
      <ul class="keys-list">
        <li v-for="(key, index) in keys" :key="index" class="key-item">
          <key-card :key-details="key" :is-active="isThisKeyActive(key)" />
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import type { Key, State } from "~/types/  core-types";

const keys = ref<Key[]>([]);
const loadedKeys = ref<Key[]>([]);
const error = ref<string | null>(null);
const commandList = <string[]>["get_ssh_keys", "get_loaded_ssh_agent_keys"];

const keysState = ref<State>({
  status: "idle",
  data: {},
  message: "Idling, waiting for input",
});

const loadedKeysState = ref<State>({
  status: "idle",
  data: {},
  message: "Idling, waiting for input",
});

onMounted(() => {
  fetchSSHKeys();
  getLoadedSSHAgentKeys();
});

const isThisKeyActive = (key: Key): boolean => {
  return loadedKeys.value.some((loadedKey) => loadedKey.keyPid === key.keyPid);
};

const formatKeys = (keys: string[]): any[] => {
  const isActive = false;
  return keys.map((key) => {
    const [keyPid, publicKey, email] = key.split(" ");
    return { keyPid, publicKey, email, isActive };
  });
};

const fetchSSHKeys = async () => {
  useStateModifier(keysState.value, "loading", "Loading...", {});
  error.value = null;
  try {
    const result = await invoke<string[]>(commandList[0]);
    console.log("result", result);

    keys.value = formatKeys(result);

    if (keys.value.length === 0) {
      useStateModifier(keysState.value, "failed", "No SSH keys found", {});
    } else {
      useStateModifier(
        keysState.value,
        "success",
        "SSH keys loaded successfully",
        {
          keys: keys.value,
        }
      );
    }
  } catch (err: any) {
    useStateModifier(keysState.value, "failed", "Could not fetch SSH Keys", {
      error: err,
    });
    error.value = err.message || "Failed to fetch SSH keys";
  }
};

const getLoadedSSHAgentKeys = async () => {
  useStateModifier(
    loadedKeysState.value,
    "loading",
    "Loading SSH Agent Keys...",
    {}
  );
  error.value = null;
  try {
    const result = await invoke<string[]>(commandList[1]);
    console.log("result", result);

    loadedKeys.value = formatKeys(result);

    if (loadedKeys.value.length === 0) {
      useStateModifier(
        loadedKeysState.value,
        "failed",
        "No SSH Agent keys found",
        {}
      );
    } else {
      useStateModifier(
        loadedKeysState.value,
        "success",
        "SSH Agent keys loaded successfully",
        {
          keys: loadedKeys.value,
        }
      );
    }
  } catch (err: any) {
    useStateModifier(
      loadedKeysState.value,
      "failed",
      "Could not fetch SSH Agent Keys",
      {
        error: err,
      }
    );
    error.value = err.message || "Failed to fetch SSH Agent keys";
  }
};
</script>

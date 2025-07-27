<template>
  <div
    class="key-card border p-4 m-2 flex flex-col h-40 rounded shadow-md text-sm overflow-hidden break-words"
  >
    <div class="flex justify-start items-center w-full">
      <p class="text-xm font-thin">Status:</p>
      <active-status-static :is-active="isActive" />
    </div>
    <div class="flex justify-between items-center">
      <div class="flex flex-col justify-center items-start">
        <p class="key-pid text-3xl">Encryption</p>
        <span class="key-pid text-xm">{{ keyDetails.keyPid }}</span>
      </div>
      <div class="flex flex-col justify-center items-end">
        <p class="key-pid text-3xl">Email</p>
        <span class="key-pid text-xm">{{ keyDetails.email }}</span>
      </div>
    </div>
    <key-card-menu
      class="mt-2"
      :public-key="keyDetails.publicKey"
      @copy="copyPublicKey(keyDetails.publicKey)"
    />
  </div>
</template>

<script setup lang="ts">
import type { Key } from "~/types/  core-types";

defineProps<{
  keyDetails: Key;
  isActive: boolean;
}>();

const copyButtonCTA = ref("Copy Public Key");
const copiedToClipboard = ref(false);

const copyPublicKey = (publicKey: string) => {
  copyButtonCTA.value = "Copied!";
  copiedToClipboard.value = true;
  navigator.clipboard
    .writeText(publicKey)
    .then(() => {
      console.log("Public key copied to clipboard");
    })
    .catch((err) => {
      console.error("Failed to copy public key: ", err);
    });

  setTimeout(() => {
    copyButtonCTA.value = "Copy Public Key";
    copiedToClipboard.value = false;
  }, 2000);
};
</script>

<style scoped>
.key-card {
  background-color: var(--secondary-bg);
  color: var(--secondary-highlight) !important;
}
</style>

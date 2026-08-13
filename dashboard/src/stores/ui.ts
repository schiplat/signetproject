import { defineStore } from "pinia";
import { ref } from "vue";

const STORAGE_KEY = "signet_sidebar_collapsed";

function readCollapsed(): boolean {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw === null) return false;
    return raw === "true";
  } catch {
    return false;
  }
}

function writeCollapsed(v: boolean) {
  try {
    localStorage.setItem(STORAGE_KEY, String(v));
  } catch {
    /* ignore */
  }
}

export const useUiStore = defineStore("ui", () => {
  const isCollapsed = ref(readCollapsed());

  function toggleSidebar() {
    isCollapsed.value = !isCollapsed.value;
    writeCollapsed(isCollapsed.value);
  }

  return { isCollapsed, toggleSidebar };
});

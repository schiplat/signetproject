import { defineStore } from "pinia";
import { computed, ref } from "vue";
import {
  login as apiLogin,
  logout as apiLogout,
  me,
  type LoginResult,
  type PublicUser,
  type UserRole,
} from "@/lib/api";

export const useAuthStore = defineStore("auth", () => {
  const user = ref<PublicUser | null>(null);
  const loaded = ref(false);

  const isAuthenticated = computed(() => !!user.value);
  const role = computed<UserRole>(() => user.value?.role ?? "member");
  const isAdmin = computed(() => role.value === "admin");
  const isManager = computed(() => role.value === "manager");
  const isStaff = computed(() => role.value === "admin" || role.value === "manager");
  const canManageUsers = computed(() => isStaff.value);
  const canDeleteUsers = computed(() => isAdmin.value);
  const canManageClients = computed(() => isStaff.value);
  const canDeleteClients = computed(() => isAdmin.value);
  const canViewAudit = computed(() => isStaff.value);
  const canManageSettings = computed(() => isAdmin.value);
  const displayName = computed(
    () => user.value?.display_name || user.value?.email || "?",
  );

  async function fetchMe() {
    try {
      const res = await me();
      user.value = res.user;
    } catch {
      user.value = null;
    } finally {
      loaded.value = true;
    }
  }

  async function login(email: string, password: string): Promise<LoginResult> {
    const res = await apiLogin(email, password);
    if (res.status === "ok") {
      user.value = res.user;
      loaded.value = true;
    }
    return res;
  }

  function completeLogin(u: PublicUser) {
    user.value = u;
    loaded.value = true;
  }

  async function logout() {
    try {
      await apiLogout();
    } finally {
      user.value = null;
    }
  }

  function setUser(u: PublicUser) {
    user.value = u;
  }

  return {
    user,
    loaded,
    isAuthenticated,
    role,
    isAdmin,
    isManager,
    isStaff,
    canManageUsers,
    canDeleteUsers,
    canManageClients,
    canDeleteClients,
    canViewAudit,
    canManageSettings,
    displayName,
    fetchMe,
    login,
    completeLogin,
    logout,
    setUser,
  };
});

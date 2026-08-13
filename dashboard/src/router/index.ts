import { createRouter, createWebHistory } from "vue-router";
import AppShell from "@/layouts/AppShell.vue";
import ActivityView from "@/views/ActivityView.vue";
import AuditLogsView from "@/views/AuditLogsView.vue";
import ClientsView from "@/views/ClientsView.vue";
import ConsentView from "@/views/ConsentView.vue";
import IntegrationsView from "@/views/IntegrationsView.vue";
import LoginView from "@/views/LoginView.vue";
import OverviewView from "@/views/OverviewView.vue";
import ResetPasswordView from "@/views/ResetPasswordView.vue";
import SettingsView from "@/views/SettingsView.vue";
import UsersView from "@/views/UsersView.vue";
import { useAuthStore } from "@/stores/auth";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/login",
      name: "login",
      component: LoginView,
      meta: { public: true },
    },
    {
      path: "/reset-password",
      name: "reset-password",
      component: ResetPasswordView,
      meta: { public: true },
    },
    {
      path: "/consent",
      name: "consent",
      component: ConsentView,
      meta: { public: true },
    },
    {
      path: "/",
      component: AppShell,
      children: [
        {
          path: "",
          redirect: () => {
            const auth = useAuthStore();
            return auth.isStaff ? "/admin/overview" : "/activity";
          },
        },
        {
          path: "activity",
          name: "activity",
          component: ActivityView,
          meta: { title: "Activity" },
        },
        {
          path: "admin/overview",
          name: "overview",
          component: OverviewView,
          meta: { title: "Overview", staff: true },
        },
        {
          path: "admin/users",
          name: "users",
          component: UsersView,
          meta: { title: "Users", staff: true },
        },
        {
          path: "admin/clients",
          name: "clients",
          component: ClientsView,
          meta: { title: "Clients", staff: true },
        },
        {
          path: "admin/audit-logs",
          name: "audit-logs",
          component: AuditLogsView,
          meta: { title: "Audit logs", staff: true },
        },
        {
          path: "admin/settings",
          name: "settings",
          component: SettingsView,
          meta: { title: "Settings", admin: true },
        },
        {
          path: "admin/integrations",
          name: "integrations",
          component: IntegrationsView,
          meta: { title: "Integrations", admin: true },
        },
      ],
    },
  ],
});

router.beforeEach(async (to) => {
  if (to.meta.public) return true;
  const auth = useAuthStore();
  if (!auth.loaded) {
    await auth.fetchMe();
  }
  if (!auth.isAuthenticated) {
    return { path: "/login", query: { return_to: to.fullPath } };
  }
  if (to.meta.admin && !auth.isAdmin) {
    return { path: auth.isStaff ? "/admin/overview" : "/activity" };
  }
  if (to.meta.staff && !auth.isStaff) {
    return { path: "/activity" };
  }
  return true;
});

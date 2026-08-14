<script setup lang="ts">
import { AppWindow, Shield, UserRound, Users } from "@lucide/vue";
import { onMounted, ref } from "vue";
import LoginTrendChart from "@/components/ui/LoginTrendChart.vue";
import PageHeader from "@/components/ui/PageHeader.vue";
import { fetchAdminStats, type AdminStats } from "@/lib/api";

const loading = ref(true);
const error = ref("");
const stats = ref<AdminStats | null>(null);

onMounted(async () => {
  try {
    stats.value = await fetchAdminStats();
  } catch (e) {
    error.value = e instanceof Error ? e.message : "Failed to load overview";
  } finally {
    loading.value = false;
  }
});

function formatShortTime(iso: string) {
  try {
    const d = new Date(iso);
    const now = Date.now();
    const diff = now - d.getTime();
    if (diff < 60_000) return "just now";
    if (diff < 3_600_000) return `${Math.floor(diff / 60_000)}m ago`;
    if (diff < 86_400_000) return `${Math.floor(diff / 3_600_000)}h ago`;
    return d.toLocaleString(undefined, { month: "short", day: "numeric", hour: "2-digit", minute: "2-digit" });
  } catch {
    return iso;
  }
}
</script>

<template>
  <div class="space-y-6">
    <PageHeader
      title="Overview"
      description="Global snapshot of Signet identity and connected applications."
    />

    <p v-if="error" class="field-error">{{ error }}</p>

    <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
      <div class="surface-card p-5">
        <div class="flex items-start justify-between gap-3">
          <div>
            <p class="type-eyebrow">Users</p>
            <p class="mt-2 text-lg font-semibold tracking-tight">
              <template v-if="loading">—</template>
              <template v-else>{{ stats?.users_total ?? 0 }}</template>
            </p>
            <p class="type-meta mt-1.5">
              <template v-if="!loading">
                {{ stats?.users_active ?? 0 }} active · {{ stats?.users_disabled ?? 0 }} disabled
              </template>
              <template v-else>Total accounts</template>
            </p>
          </div>
          <span class="flex h-9 w-9 items-center justify-center rounded-xl bg-muted text-muted-foreground">
            <Users class="h-4 w-4" />
          </span>
        </div>
      </div>

      <div class="surface-card p-5">
        <div class="flex items-start justify-between gap-3">
          <div>
            <p class="type-eyebrow">Admins</p>
            <p class="mt-2 text-lg font-semibold tracking-tight">
              <template v-if="loading">—</template>
              <template v-else>{{ stats?.users_admin ?? 0 }}</template>
            </p>
            <p class="type-meta mt-1.5">Active administrators</p>
          </div>
          <span class="flex h-9 w-9 items-center justify-center rounded-xl bg-muted text-muted-foreground">
            <Shield class="h-4 w-4" />
          </span>
        </div>
      </div>

      <div class="surface-card p-5">
        <div class="flex items-start justify-between gap-3">
          <div>
            <p class="type-eyebrow">Managers</p>
            <p class="mt-2 text-lg font-semibold tracking-tight">
              <template v-if="loading">—</template>
              <template v-else>{{ stats?.users_manager ?? 0 }}</template>
            </p>
            <p class="type-meta mt-1.5">Active managers</p>
          </div>
          <span class="flex h-9 w-9 items-center justify-center rounded-xl bg-muted text-muted-foreground">
            <UserRound class="h-4 w-4" />
          </span>
        </div>
      </div>

      <div class="surface-card p-5">
        <div class="flex items-start justify-between gap-3">
          <div>
            <p class="type-eyebrow">Clients</p>
            <p class="mt-2 text-lg font-semibold tracking-tight">
              <template v-if="loading">—</template>
              <template v-else>
                {{ stats?.clients_enabled ?? 0 }}
                <span class="text-sm font-normal text-muted-foreground">
                  / {{ stats?.clients_total ?? 0 }}
                </span>
              </template>
            </p>
            <p class="type-meta mt-1.5">Enabled / Total OIDC apps</p>
          </div>
          <span class="flex h-9 w-9 items-center justify-center rounded-xl bg-muted text-muted-foreground">
            <AppWindow class="h-4 w-4" />
          </span>
        </div>
      </div>
    </div>

    <section class="space-y-3">
      <div class="flex flex-wrap items-end justify-between gap-3">
        <div>
          <h2 class="text-sm font-semibold tracking-tight">User logins</h2>
          <p class="type-meta mt-1">
            Successful Signet sign-ins (auth.login) over the last 30 days — daily, 7-day, and 30-day
            rolling totals on one chart.
          </p>
        </div>
        <div
          v-if="!loading && stats"
          class="flex flex-wrap gap-x-4 gap-y-1 text-[11px] text-muted-foreground"
        >
          <span>
            Now · 24h
            <span class="font-medium text-foreground tabular-nums">{{ stats.logins_24h }}</span>
          </span>
          <span>
            7d
            <span class="font-medium text-foreground tabular-nums">{{ stats.logins_7d }}</span>
          </span>
          <span>
            30d
            <span class="font-medium text-foreground tabular-nums">{{ stats.logins_30d }}</span>
          </span>
        </div>
      </div>

      <div class="grid gap-3 lg:grid-cols-5">
        <div class="surface-card flex flex-col justify-center p-5 lg:col-span-3">
          <div v-if="loading" class="flex h-[280px] items-center justify-center text-sm text-muted-foreground">
            Loading…
          </div>
          <div
            v-else-if="!stats?.login_trend?.length"
            class="flex h-[280px] items-center justify-center text-sm text-muted-foreground"
          >
            No login trend data
          </div>
          <LoginTrendChart v-else :points="stats.login_trend" />
        </div>

        <div class="surface-card flex flex-col overflow-hidden lg:col-span-2">
          <div class="flex shrink-0 items-center justify-between border-b border-border/30 px-5 py-3">
            <p class="text-xs font-semibold uppercase tracking-[0.06em] text-muted-foreground">
              Recent logins
            </p>
            <RouterLink
              to="/audit-logs"
              class="text-[11px] font-medium text-muted-foreground transition-colors hover:text-foreground"
            >
              View all →
            </RouterLink>
          </div>
          <div v-if="loading" class="py-10 text-center text-sm text-muted-foreground">Loading…</div>
          <div
            v-else-if="!stats?.recent_logins?.length"
            class="flex flex-1 items-center justify-center py-10 text-center text-sm text-muted-foreground"
          >
            No login events in the last 7 days
          </div>
          <ul v-else class="max-h-[300px] divide-y divide-border/20 overflow-y-auto">
            <li v-for="(row, idx) in stats.recent_logins" :key="idx" class="px-5 py-3">
              <div class="flex items-center justify-between gap-3">
                <p class="min-w-0 truncate text-xs font-medium">{{ row.actor_email || "—" }}</p>
                <p class="whitespace-nowrap text-[11px] text-muted-foreground">
                  {{ formatShortTime(row.created_at) }}
                </p>
              </div>
              <p class="mt-1 font-mono text-[11px] text-muted-foreground">
                {{ [row.ip, row.browser, row.os].filter(Boolean).join(" · ") || "—" }}
              </p>
            </li>
          </ul>
          <div class="shrink-0 border-t border-border/30 px-5 py-2.5 text-center">
            <RouterLink
              to="/audit-logs"
              class="text-[11px] font-medium text-muted-foreground transition-colors hover:text-foreground"
            >
              Open audit logs
            </RouterLink>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

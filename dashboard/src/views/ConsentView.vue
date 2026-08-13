<script setup lang="ts">
import { computed, ref } from "vue";
import { useRoute } from "vue-router";
import UiButton from "@/components/ui/UiButton.vue";

const route = useRoute();
const loading = ref(false);
const error = ref("");

const q = (key: string) => (typeof route.query[key] === "string" ? route.query[key] : "");

const clientId = q("client_id");
const redirectUri = q("redirect_uri");
const scope = q("scope") || "openid";
const state = q("state");
const nonce = q("nonce");
const codeChallenge = q("code_challenge");
const codeChallengeMethod = q("code_challenge_method") || "S256";

const SCOPE_LABELS: Record<string, string> = {
  openid: "Sign you in",
  profile: "View your profile (name)",
  email: "View your email address",
  phone: "View your phone number",
  groups: "View your group memberships",
};

const requestedScopes = computed(() =>
  scope
    .split(/\s+/)
    .filter(Boolean)
    .map((s) => ({ key: s, label: SCOPE_LABELS[s] ?? s })),
);

// Scopes the client is allowed to have but did not request — offered as opt-in.
const optionalScopes = computed(() =>
  (q("optional_scopes") || "")
    .split(/\s+/)
    .filter(Boolean)
    .map((s) => ({ key: s, label: SCOPE_LABELS[s] ?? s })),
);

const optionalSelected = ref<Set<string>>(new Set());

function toggleOptional(key: string) {
  const next = new Set(optionalSelected.value);
  if (next.has(key)) next.delete(key);
  else next.add(key);
  optionalSelected.value = next;
}

async function submit(allow: boolean) {
  loading.value = true;
  error.value = "";
  try {
    const res = await fetch("/oauth/consent", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      credentials: "include",
      redirect: "manual",
      body: JSON.stringify({
        client_id: clientId,
        redirect_uri: redirectUri,
        scope,
        optional_scopes: [...optionalSelected.value].join(" "),
        state: state || undefined,
        nonce: nonce || undefined,
        code_challenge: codeChallenge,
        code_challenge_method: codeChallengeMethod,
        allow,
      }),
    });
    const loc = res.headers.get("location");
    if (loc) {
      window.location.href = loc;
    } else {
      error.value = "Unexpected response from the server";
    }
  } catch (e) {
    error.value = e instanceof Error ? e.message : "Request failed";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="flex min-h-screen items-center justify-center bg-background px-4">
    <div class="w-full max-w-sm space-y-6">
      <div class="space-y-1 text-center">
        <div class="mb-3 flex justify-center">
          <span
            class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary text-sm font-semibold tracking-wide text-primary-foreground"
          >
            Sg
          </span>
        </div>
        <h1 class="text-lg font-semibold tracking-tight">Authorize application</h1>
        <p class="type-meta">Signet is asking for your consent</p>
      </div>

      <div class="rounded-2xl border border-border/50 bg-card p-5 shadow-sm">
        <p class="text-sm font-medium">{{ clientId || "Application" }}</p>
        <p class="type-meta mt-0.5 break-all text-xs">{{ redirectUri }}</p>

        <p class="mt-5 mb-2 text-[12px] font-medium text-muted-foreground">
          This application is requesting access to:
        </p>
        <ul class="space-y-1.5">
          <li
            v-for="s in requestedScopes"
            :key="s.key"
            class="flex items-start gap-2.5 text-sm"
          >
            <span class="mt-1.5 h-1.5 w-1.5 shrink-0 rounded-full bg-primary" />
            <span>
              {{ s.label }}
              <span class="ml-1 font-mono text-[10px] text-muted-foreground">{{ s.key }}</span>
            </span>
          </li>
        </ul>

        <template v-if="optionalScopes.length">
          <p class="mt-5 mb-2 text-[12px] font-medium text-muted-foreground">
            You can also grant:
          </p>
          <ul class="space-y-1.5">
            <li
              v-for="s in optionalScopes"
              :key="s.key"
              class="flex items-start gap-2.5 text-sm"
            >
              <input
                type="checkbox"
                class="mt-0.5 rounded"
                :checked="optionalSelected.has(s.key)"
                @change="toggleOptional(s.key)"
              />
              <span>
                {{ s.label }}
                <span class="ml-1 font-mono text-[10px] text-muted-foreground">{{ s.key }}</span>
              </span>
            </li>
          </ul>
        </template>
      </div>

      <div v-if="error" class="field-error">{{ error }}</div>

      <div class="flex gap-3">
        <UiButton variant="ghost" class="flex-1" :disabled="loading" @click="submit(false)">
          Deny
        </UiButton>
        <UiButton class="flex-1" :disabled="loading" @click="submit(true)">
          {{ loading ? "Please wait…" : "Allow" }}
        </UiButton>
      </div>

      <p class="text-center text-xs text-muted-foreground">
        You can review or revoke this consent from the application later.
      </p>
    </div>
  </div>
</template>

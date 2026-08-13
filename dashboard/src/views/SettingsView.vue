<script setup lang="ts">
import { onMounted, ref } from "vue";
import PageHeader from "@/components/ui/PageHeader.vue";
import UiButton from "@/components/ui/UiButton.vue";
import { fetchMfaSettings, updateMfaSettings } from "@/lib/api";

const loading = ref(true);
const saving = ref(false);
const error = ref("");
const saved = ref(false);
const requiredGlobally = ref(false);

onMounted(async () => {
  try {
    const res = await fetchMfaSettings();
    requiredGlobally.value = res.required_globally;
  } catch (e) {
    error.value = e instanceof Error ? e.message : "Failed to load settings";
  } finally {
    loading.value = false;
  }
});

async function onSave() {
  error.value = "";
  saved.value = false;
  saving.value = true;
  try {
    const res = await updateMfaSettings({ required_globally: requiredGlobally.value });
    requiredGlobally.value = res.required_globally;
    saved.value = true;
  } catch (e) {
    error.value = e instanceof Error ? e.message : "Save failed";
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <div class="space-y-6">
    <PageHeader
      title="Settings"
      description="Global Signet configuration. Admin only."
    />

    <p v-if="error" class="field-error">{{ error }}</p>
    <div v-if="loading" class="py-12 text-center text-sm text-muted-foreground">Loading…</div>

    <section
      v-else
      class="max-w-xl space-y-5 rounded-xl bg-card p-6 shadow-sm"
    >
      <div>
        <h2 class="text-sm font-semibold tracking-tight">Security</h2>
        <p class="mt-1 text-xs text-muted-foreground">
          Controls whether users without TOTP must enroll before signing in. Users who already
          enabled MFA always verify on login.
        </p>
      </div>

      <label class="flex items-start gap-3 rounded-lg border border-border/40 px-4 py-3">
        <input v-model="requiredGlobally" type="checkbox" class="mt-0.5 rounded" />
        <span>
          <span class="block text-sm font-medium">Require MFA for all users</span>
          <span class="mt-0.5 block text-xs text-muted-foreground">
            When on, every account without an authenticator must enroll at next login. Per-user
            Require MFA still applies when this is off.
          </span>
        </span>
      </label>

      <div class="flex items-center gap-3">
        <UiButton size="sm" :disabled="saving" @click="onSave">
          {{ saving ? "Saving…" : "Save" }}
        </UiButton>
        <span v-if="saved" class="text-xs text-muted-foreground">Saved</span>
      </div>
    </section>
  </div>
</template>

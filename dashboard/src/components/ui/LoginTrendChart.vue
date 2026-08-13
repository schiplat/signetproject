<script setup lang="ts">
import { computed, ref } from "vue";
import type { LoginTrendPoint } from "@/lib/api";

const props = defineProps<{
  points: LoginTrendPoint[];
}>();

type SeriesKey = "logins_1d" | "logins_7d" | "logins_30d";

const SERIES: { key: SeriesKey; label: string; color: string }[] = [
  { key: "logins_1d", label: "24h (daily)", color: "hsl(25 92% 46%)" },
  { key: "logins_7d", label: "7-day rolling", color: "hsl(152 72% 33%)" },
  { key: "logins_30d", label: "30-day rolling", color: "hsl(219 90% 52%)" },
];

const W = 720;
const H = 260;
const PAD = { top: 16, right: 12, bottom: 28, left: 36 };

const hoverIdx = ref<number | null>(null);
const visible = ref<Record<SeriesKey, boolean>>({
  logins_1d: true,
  logins_7d: true,
  logins_30d: true,
});

const visibleSeries = computed(() => SERIES.filter((s) => visible.value[s.key]));

function toggleSeries(key: SeriesKey) {
  visible.value = { ...visible.value, [key]: !visible.value[key] };
}

const maxY = computed(() => {
  let m = 1;
  for (const p of props.points) {
    for (const s of visibleSeries.value) m = Math.max(m, p[s.key]);
  }
  return m;
});

const yTicks = computed(() => {
  const max = maxY.value;
  const step = niceStep(max);
  const ticks: number[] = [];
  for (let v = 0; v <= max; v += step) ticks.push(v);
  if (ticks[ticks.length - 1] < max) ticks.push(ticks[ticks.length - 1] + step);
  return ticks;
});

const chartMax = computed(() => yTicks.value[yTicks.value.length - 1] || 1);

const plotW = W - PAD.left - PAD.right;
const plotH = H - PAD.top - PAD.bottom;

function xAt(i: number) {
  const n = Math.max(props.points.length - 1, 1);
  return PAD.left + (i / n) * plotW;
}

function yAt(v: number) {
  return PAD.top + plotH - (v / chartMax.value) * plotH;
}

function linePath(key: SeriesKey) {
  if (!props.points.length) return "";
  return props.points
    .map((p, i) => `${i === 0 ? "M" : "L"} ${xAt(i).toFixed(2)} ${yAt(p[key]).toFixed(2)}`)
    .join(" ");
}

function formatDay(day: string) {
  const d = new Date(`${day}T00:00:00Z`);
  if (Number.isNaN(d.getTime())) return day;
  return d.toLocaleDateString(undefined, { month: "short", day: "numeric", timeZone: "UTC" });
}

function niceStep(max: number) {
  if (max <= 4) return 1;
  const raw = max / 4;
  const pow = 10 ** Math.floor(Math.log10(raw));
  const n = raw / pow;
  const nice = n <= 1 ? 1 : n <= 2 ? 2 : n <= 5 ? 5 : 10;
  return nice * pow;
}

function onMove(e: MouseEvent) {
  const svg = e.currentTarget as SVGSVGElement;
  const rect = svg.getBoundingClientRect();
  const x = ((e.clientX - rect.left) / rect.width) * W;
  if (x < PAD.left || x > W - PAD.right || !props.points.length) {
    hoverIdx.value = null;
    return;
  }
  const n = Math.max(props.points.length - 1, 1);
  const i = Math.round(((x - PAD.left) / plotW) * n);
  hoverIdx.value = Math.min(Math.max(i, 0), props.points.length - 1);
}

function onLeave() {
  hoverIdx.value = null;
}

const hover = computed(() => {
  if (hoverIdx.value == null) return null;
  return props.points[hoverIdx.value] ?? null;
});

const xLabels = computed(() => {
  const pts = props.points;
  if (pts.length === 0) return [];
  const idxs = new Set<number>([0, pts.length - 1]);
  if (pts.length > 2) idxs.add(Math.floor((pts.length - 1) / 2));
  if (pts.length > 8) {
    idxs.add(Math.floor((pts.length - 1) / 4));
    idxs.add(Math.floor(((pts.length - 1) * 3) / 4));
  }
  return [...idxs].sort((a, b) => a - b).map((i) => ({ i, label: formatDay(pts[i].day) }));
});
</script>

<template>
  <div class="space-y-3">
    <div class="flex flex-wrap items-center gap-x-4 gap-y-2">
      <button
        v-for="s in SERIES"
        :key="s.key"
        type="button"
        class="group flex cursor-pointer items-center gap-2 text-xs transition-colors"
        :class="visible[s.key] ? 'text-muted-foreground' : 'text-muted-foreground/40'"
        :aria-pressed="visible[s.key]"
        @click="toggleSeries(s.key)"
      >
        <span
          class="h-0.5 w-4 rounded-full transition-opacity"
          :style="{ background: s.color }"
          :class="visible[s.key] ? 'opacity-100' : 'opacity-30'"
        />
        <span :class="visible[s.key] ? '' : 'line-through'">{{ s.label }}</span>
      </button>
    </div>

    <div class="relative">
      <svg
        class="h-[260px] w-full select-none"
        :viewBox="`0 0 ${W} ${H}`"
        role="img"
        aria-label="Login trend: daily, 7-day rolling, and 30-day rolling"
        @mousemove="onMove"
        @mouseleave="onLeave"
      >
        <rect
          :x="PAD.left"
          :y="PAD.top"
          :width="plotW"
          :height="plotH"
          class="fill-muted/20"
          rx="6"
        />

        <g v-for="tick in yTicks" :key="tick">
          <line
            :x1="PAD.left"
            :x2="W - PAD.right"
            :y1="yAt(tick)"
            :y2="yAt(tick)"
            class="stroke-border/50"
            stroke-width="1"
          />
          <text
            :x="PAD.left - 8"
            :y="yAt(tick) + 3"
            text-anchor="end"
            class="fill-muted-foreground"
            font-size="10"
          >
            {{ tick }}
          </text>
        </g>

        <path
          v-for="s in visibleSeries"
          :key="s.key"
          :d="linePath(s.key)"
          fill="none"
          :stroke="s.color"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="transition-[opacity] duration-150"
          :opacity="hoverIdx == null ? 1 : 0.45"
        />

        <template v-if="hoverIdx != null && hover">
          <line
            :x1="xAt(hoverIdx)"
            :x2="xAt(hoverIdx)"
            :y1="PAD.top"
            :y2="PAD.top + plotH"
            class="stroke-foreground/25"
            stroke-width="1"
            stroke-dasharray="3 3"
          />
          <circle
            v-for="s in visibleSeries"
            :key="`dot-${s.key}`"
            :cx="xAt(hoverIdx)"
            :cy="yAt(hover[s.key])"
            r="3.5"
            :fill="s.color"
            class="stroke-card"
            stroke-width="1.5"
          />
        </template>

        <text
          v-for="lab in xLabels"
          :key="lab.i"
          :x="xAt(lab.i)"
          :y="H - 8"
          text-anchor="middle"
          class="fill-muted-foreground"
          font-size="10"
        >
          {{ lab.label }}
        </text>
      </svg>

      <div
        v-if="hover && hoverIdx != null"
        class="pointer-events-none absolute top-2 z-10 min-w-[9.5rem] rounded-lg border border-border/50 bg-card px-3 py-2 shadow-sm"
        :style="{
          left: `clamp(0.5rem, ${(xAt(hoverIdx) / W) * 100}% , calc(100% - 10rem))`,
        }"
      >
        <p class="text-[11px] font-semibold tracking-tight">{{ formatDay(hover.day) }}</p>
        <ul class="mt-1.5 space-y-1">
          <li
            v-for="s in visibleSeries"
            :key="s.key"
            class="flex items-center justify-between gap-4 text-[11px]"
          >
            <span class="flex items-center gap-1.5 text-muted-foreground">
              <span class="h-1.5 w-1.5 rounded-full" :style="{ background: s.color }" />
              {{ s.label }}
            </span>
            <span class="font-medium tabular-nums">{{ hover[s.key] }}</span>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

import QRCode from "qrcode";
import { ref, watch, type Ref } from "vue";

export function useQrDataUrl(text: Ref<string>) {
  const dataUrl = ref("");
  const error = ref("");

  watch(
    text,
    async (value) => {
      dataUrl.value = "";
      error.value = "";
      if (!value) return;
      try {
        dataUrl.value = await QRCode.toDataURL(value, {
          width: 200,
          margin: 1,
          color: { dark: "#1c1917", light: "#ffffff" },
        });
      } catch (e) {
        error.value = e instanceof Error ? e.message : "QR failed";
      }
    },
    { immediate: true },
  );

  return { dataUrl, error };
}

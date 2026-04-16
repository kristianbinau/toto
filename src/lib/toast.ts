export type ToastSink = (opts: {
  title: string;
  description?: string;
  color?: "error" | "success" | "info" | "warning" | "primary" | "neutral";
  icon?: string;
}) => void;

let sink: ToastSink | null = null;

export function setToastSink(fn: ToastSink | null): void {
  sink = fn;
}

export function errorToast(title: string, error: unknown): void {
  const description =
    error instanceof Error ? error.message : typeof error === "string" ? error : String(error);
  if (sink) {
    sink({ title, description, color: "error", icon: "i-lucide-triangle-alert" });
  } else {
    console.error(title, error);
  }
}

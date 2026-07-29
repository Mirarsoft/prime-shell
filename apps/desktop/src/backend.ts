import { Channel, invoke } from "@tauri-apps/api/core";
import {
  appErrorSchema,
  backendStatusSchema,
  cancelReceiptSchema,
  echoResponseSchema,
  runtimeProbeConfigSchema,
  taskSnapshotSchema,
  type AppError,
  type BackendStatus,
  type CancelReceipt,
  type EchoResponse,
  type RuntimeProbeConfig,
  type TaskSnapshot,
} from "./contracts";

export async function getBackendStatus(): Promise<BackendStatus> {
  return backendStatusSchema.parse(await invoke("backend_status"));
}

export async function echoText(text: string): Promise<EchoResponse> {
  return echoResponseSchema.parse(await invoke("echo_text", { text }));
}

export async function startCount(
  countTo: number,
  intervalMs: number,
  timeoutMs: number,
  onTaskEvent: (event: TaskSnapshot) => void,
): Promise<TaskSnapshot> {
  const onEvent = new Channel<unknown>();
  onEvent.onmessage = (value) => {
    onTaskEvent(taskSnapshotSchema.parse(value));
  };
  return taskSnapshotSchema.parse(
    await invoke("start_count", {
      countTo,
      intervalMs,
      timeoutMs,
      onEvent,
    }),
  );
}

export async function cancelTask(taskId: string): Promise<CancelReceipt> {
  return cancelReceiptSchema.parse(await invoke("cancel_task", { taskId }));
}

export async function getTaskStatus(taskId: string): Promise<TaskSnapshot> {
  return taskSnapshotSchema.parse(await invoke("task_status", { taskId }));
}

export async function recoverBackend(): Promise<BackendStatus> {
  return backendStatusSchema.parse(await invoke("recover_backend"));
}

export async function getRuntimeProbeConfig(): Promise<RuntimeProbeConfig> {
  return runtimeProbeConfigSchema.parse(await invoke("runtime_probe_config"));
}

export async function writeRuntimeEvidence(
  evidence: Record<string, unknown>,
): Promise<void> {
  await invoke("write_runtime_evidence", { evidence });
}

export function toSafeError(value: unknown): AppError {
  const parsed = appErrorSchema.safeParse(value);
  if (parsed.success) {
    return parsed.data;
  }

  return {
    code: "INTERNAL_ERROR",
    message: "The request could not be completed.",
    traceId: "frontend-unmapped",
  };
}

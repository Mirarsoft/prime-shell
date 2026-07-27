import { invoke } from "@tauri-apps/api/core";
import {
  appErrorSchema,
  backendStatusSchema,
  echoResponseSchema,
  runtimeProbeConfigSchema,
  type AppError,
  type BackendStatus,
  type EchoResponse,
  type RuntimeProbeConfig,
} from "./contracts";

export async function getBackendStatus(): Promise<BackendStatus> {
  return backendStatusSchema.parse(await invoke("backend_status"));
}

export async function echoText(text: string): Promise<EchoResponse> {
  return echoResponseSchema.parse(await invoke("echo_text", { text }));
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
    message: "The echo request could not be completed.",
    traceId: "frontend-unmapped",
  };
}

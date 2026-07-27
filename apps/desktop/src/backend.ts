import { invoke } from "@tauri-apps/api/core";
import {
  appErrorSchema,
  backendStatusSchema,
  echoResponseSchema,
  type AppError,
  type BackendStatus,
  type EchoResponse,
} from "./contracts";

export async function getBackendStatus(): Promise<BackendStatus> {
  return backendStatusSchema.parse(await invoke("backend_status"));
}

export async function echoText(text: string): Promise<EchoResponse> {
  return echoResponseSchema.parse(await invoke("echo_text", { text }));
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


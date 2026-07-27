import { z } from "zod";

export const appErrorSchema = z.object({
  code: z.enum([
    "VALIDATION_ERROR",
    "BACKEND_UNAVAILABLE",
    "BACKEND_PROTOCOL_MISMATCH",
    "PROTOCOL_ERROR",
    "RESOURCE_EXHAUSTED",
    "IO_ERROR",
    "INTERNAL_ERROR",
  ]),
  message: z.string().min(1).max(256),
  traceId: z.string().min(1).max(128),
});

export const backendStatusSchema = z.object({
  ready: z.boolean(),
  backendVersion: z.string().max(64).nullable(),
});

export const echoResponseSchema = z.object({
  text: z.string(),
  traceId: z.string().min(1).max(128),
});

export const runtimeProbeConfigSchema = z.object({
  enabled: z.boolean(),
  evidencePath: z.string().nullable(),
});

export type AppError = z.infer<typeof appErrorSchema>;
export type BackendStatus = z.infer<typeof backendStatusSchema>;
export type EchoResponse = z.infer<typeof echoResponseSchema>;
export type RuntimeProbeConfig = z.infer<typeof runtimeProbeConfigSchema>;

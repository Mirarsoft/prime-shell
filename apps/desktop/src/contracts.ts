import { z } from "zod";

export const appErrorSchema = z.object({
  code: z.enum([
    "VALIDATION_ERROR",
    "CONFLICT",
    "NOT_FOUND",
    "BACKEND_UNAVAILABLE",
    "BACKEND_CRASHED",
    "BACKEND_PROTOCOL_MISMATCH",
    "PROTOCOL_ERROR",
    "RESOURCE_EXHAUSTED",
    "IO_ERROR",
    "TIMEOUT",
    "CANCELLED",
    "INTERRUPTED",
    "INTERNAL_ERROR",
  ]),
  message: z.string().min(1).max(256),
  traceId: z.string().min(1).max(128),
});

export const backendLifecycleSchema = z.enum([
  "Stopped",
  "Starting",
  "Ready",
  "Busy",
  "Restarting",
  "Faulted",
  "Stopping",
]);

export const backendStatusSchema = z.object({
  state: backendLifecycleSchema,
  backendVersion: z.string().max(64).nullable(),
  restartCount: z.number().int().min(0).max(1),
  circuitOpen: z.boolean(),
  activeTaskId: z.string().min(1).max(128).nullable(),
  lastError: appErrorSchema.nullable(),
});

export const taskLifecycleSchema = z.enum([
  "Queued",
  "Running",
  "Cancelling",
  "Succeeded",
  "Failed",
  "Cancelled",
  "TimedOut",
  "Interrupted",
]);

export const taskProgressSchema = z.object({
  current: z.number().int().min(0),
  target: z.number().int().min(1).max(10_000),
});

export const taskSnapshotSchema = z.object({
  taskId: z.string().min(1).max(128),
  traceId: z.string().min(1).max(128),
  operation: z.enum(["spike.count", "spike.crash", "spike.hang"]),
  state: taskLifecycleSchema,
  sequence: z.number().int().min(0),
  progress: taskProgressSchema.nullable(),
  result: z.record(z.string(), z.unknown()).nullable(),
  error: appErrorSchema.nullable(),
});

export const cancelReceiptSchema = z.object({
  taskId: z.string().min(1).max(128),
  accepted: z.boolean(),
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
export type BackendLifecycle = z.infer<typeof backendLifecycleSchema>;
export type BackendStatus = z.infer<typeof backendStatusSchema>;
export type TaskLifecycle = z.infer<typeof taskLifecycleSchema>;
export type TaskSnapshot = z.infer<typeof taskSnapshotSchema>;
export type CancelReceipt = z.infer<typeof cancelReceiptSchema>;
export type EchoResponse = z.infer<typeof echoResponseSchema>;
export type RuntimeProbeConfig = z.infer<typeof runtimeProbeConfigSchema>;

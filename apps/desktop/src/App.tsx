import { useEffect, useRef, useState } from "react";
import {
  Button,
  Card,
  Field,
  FluentProvider,
  Input,
  ProgressBar,
  Spinner,
  Text,
  Title1,
  Title3,
  webDarkTheme,
  webLightTheme,
} from "@fluentui/react-components";
import {
  cancelTask,
  echoText,
  getBackendStatus,
  getRuntimeProbeConfig,
  recoverBackend,
  startCount,
  toSafeError,
  writeRuntimeEvidence,
} from "./backend";
import type {
  AppError,
  BackendStatus,
  TaskLifecycle,
  TaskSnapshot,
} from "./contracts";
import "./app.css";

const RUNTIME_PROBE_TEXT = "Hello — مرحبا — こんにちは 👋";
const TERMINAL_STATES = new Set<TaskLifecycle>([
  "Succeeded",
  "Failed",
  "Cancelled",
  "TimedOut",
  "Interrupted",
]);

function preferredTheme() {
  return window.matchMedia("(prefers-color-scheme: dark)").matches
    ? webDarkTheme
    : webLightTheme;
}

function isTerminal(task: TaskSnapshot | null): boolean {
  return task !== null && TERMINAL_STATES.has(task.state);
}

export default function App() {
  const [backend, setBackend] = useState<BackendStatus | null>(null);
  const [checking, setChecking] = useState(true);
  const [text, setText] = useState(RUNTIME_PROBE_TEXT);
  const [echoResult, setEchoResult] = useState("");
  const [echoBusy, setEchoBusy] = useState(false);
  const [countTo, setCountTo] = useState("20");
  const [intervalMs, setIntervalMs] = useState("100");
  const [task, setTask] = useState<TaskSnapshot | null>(null);
  const [error, setError] = useState<AppError | null>(null);
  const runtimeProbeStarted = useRef(false);
  const cspViolations = useRef<string[]>([]);

  const backendReady = backend?.state === "Ready";
  const taskActive = task !== null && !isTerminal(task);

  async function refreshBackend() {
    const status = await getBackendStatus();
    setBackend(status);
    return status;
  }

  function recordTaskEvent(event: TaskSnapshot) {
    setTask(event);
    if (TERMINAL_STATES.has(event.state)) {
      void refreshBackend().catch((reason: unknown) => {
        setError(toSafeError(reason));
      });
    }
  }

  useEffect(() => {
    const recordViolation = (event: SecurityPolicyViolationEvent) => {
      cspViolations.current.push(
        `${event.violatedDirective}:${event.blockedURI}`,
      );
    };
    document.addEventListener("securitypolicyviolation", recordViolation);
    return () => {
      document.removeEventListener("securitypolicyviolation", recordViolation);
    };
  }, []);

  useEffect(() => {
    void refreshBackend()
      .catch((reason: unknown) => setError(toSafeError(reason)))
      .finally(() => setChecking(false));
  }, []);

  async function submitEcho() {
    setEchoBusy(true);
    setError(null);
    setEchoResult("");
    try {
      const response = await echoText(text);
      setEchoResult(response.text);
    } catch (reason) {
      setError(toSafeError(reason));
    } finally {
      setEchoBusy(false);
    }
  }

  async function beginCount() {
    setError(null);
    setTask(null);
    try {
      const accepted = await startCount(
        Number(countTo),
        Number(intervalMs),
        30_000,
        recordTaskEvent,
      );
      setTask((current) =>
        current !== null && current.taskId === accepted.taskId
          ? current
          : accepted,
      );
      setBackend((current) =>
        current === null
          ? current
          : {
              ...current,
              state: "Busy",
              activeTaskId: accepted.taskId,
            },
      );
    } catch (reason) {
      setError(toSafeError(reason));
      await refreshBackend().catch(() => undefined);
    }
  }

  async function requestCancel() {
    if (task === null) {
      return;
    }
    setError(null);
    try {
      await cancelTask(task.taskId);
      setTask((current) =>
        current === null ? current : { ...current, state: "Cancelling" },
      );
    } catch (reason) {
      setError(toSafeError(reason));
    }
  }

  async function requestRecovery() {
    setError(null);
    try {
      setBackend(await recoverBackend());
    } catch (reason) {
      setError(toSafeError(reason));
      await refreshBackend().catch(() => undefined);
    }
  }

  useEffect(() => {
    if (runtimeProbeStarted.current) {
      return;
    }
    runtimeProbeStarted.current = true;

    void (async () => {
      const probe = await getRuntimeProbeConfig();
      if (!probe.enabled) {
        return;
      }

      const status = await refreshBackend();
      let unicodeResult = "";
      let safeError = "";
      const countEvents: Array<TaskSnapshot & { observedAtMs: number }> = [];
      let terminalObservedAtMs = 0;

      setText(RUNTIME_PROBE_TEXT);
      setError(null);
      setEchoResult("");
      if (status.state === "Ready") {
        const response = await echoText(RUNTIME_PROBE_TEXT);
        unicodeResult = response.text;
        setEchoResult(response.text);
      }

      let resolveTerminal:
        | ((terminal: TaskSnapshot) => void)
        | undefined;
      const terminalPromise = new Promise<TaskSnapshot>((resolve) => {
        resolveTerminal = resolve;
      });
      const countAccepted = await startCount(100, 25, 10_000, (event) => {
        const observedAtMs = performance.now();
        countEvents.push({ ...event, observedAtMs });
        recordTaskEvent(event);
        if (TERMINAL_STATES.has(event.state)) {
          terminalObservedAtMs = observedAtMs;
          resolveTerminal?.(event);
        }
      });
      const countAcceptedAtMs = performance.now();
      setTask(countAccepted);
      await new Promise((resolve) => setTimeout(resolve, 240));
      const cancellationStartedAtMs = performance.now();
      const countCancelAcknowledgement = await cancelTask(
        countAccepted.taskId,
      );
      const cancelAcknowledgedAtMs = performance.now();
      const countTerminal = await Promise.race([
        terminalPromise,
        new Promise<never>((_, reject) => {
          window.setTimeout(
            () => reject(new Error("Native count probe timed out.")),
            8_000,
          );
        }),
      ]);
      const countStatus = await refreshBackend();
      const progressTimes = countEvents
        .filter((event) => event.progress !== null)
        .map((event) => event.observedAtMs);
      const progressIntervals = progressTimes
        .slice(1)
        .map((value, index) => value - progressTimes[index]);

      try {
        await echoText("x".repeat(262_145));
      } catch (reason) {
        const parsed = toSafeError(reason);
        safeError = `${parsed.code}: ${parsed.message} (${parsed.traceId})`;
        setError(parsed);
      }

      await new Promise((resolve) => requestAnimationFrame(resolve));

      await writeRuntimeEvidence({
        status: "passed",
        windowRendered:
          window.innerWidth > 0 &&
          window.innerHeight > 0 &&
          document.visibilityState === "visible",
        viewport: {
          width: window.innerWidth,
          height: window.innerHeight,
        },
        fluentRendered: Boolean(document.querySelector(".fui-FluentProvider")),
        releaseCspViolationCount: cspViolations.current.length,
        releaseCspViolations: cspViolations.current,
        backendReady: status.state === "Ready",
        backendState: status.state,
        backendVersion: status.backendVersion,
        countAcceptedAtMs,
        countTaskId: countTerminal.taskId,
        countTerminalState: countTerminal.state,
        countTerminalTraceId: countTerminal.traceId,
        countCancelAccepted: countCancelAcknowledgement.accepted,
        cancelAcknowledgementMs:
          cancelAcknowledgedAtMs - cancellationStartedAtMs,
        countTerminalCount: countEvents.filter((event) =>
          TERMINAL_STATES.has(event.state),
        ).length,
        countSequences: countEvents.map((event) => event.sequence),
        countSequencesMonotonic: countEvents.every(
          (event, index) =>
            index === 0 || event.sequence > countEvents[index - 1].sequence,
        ),
        countProgressObserved: progressTimes.length > 0,
        uiProgressMinimumIntervalMs:
          progressIntervals.length === 0
            ? null
            : Math.min(...progressIntervals),
        cooperativeStopMs:
          terminalObservedAtMs - cancellationStartedAtMs,
        backendStateAfterCount: countStatus.state,
        unicodeInput: RUNTIME_PROBE_TEXT,
        unicodeOutput: unicodeResult,
        unicodeExactMatch: unicodeResult === RUNTIME_PROBE_TEXT,
        renderedText: document.body.innerText,
        safeErrorPath: safeError,
      });
    })().catch((reason: unknown) => setError(toSafeError(reason)));
  }, []);

  return (
    <FluentProvider theme={preferredTheme()}>
      <main className="app">
        <Card className="card">
          <header>
            <Title1>Packaged Backend Resilience</Title1>
            <div className="status" aria-live="polite" aria-atomic="true">
              {checking ? (
                <Spinner size="tiny" label="Checking backend" />
              ) : (
                <Text weight="semibold">
                  Backend: {backend?.state ?? "Unavailable"}
                </Text>
              )}
            </div>
          </header>

          <section className="task-panel" aria-labelledby="count-heading">
            <Title3 id="count-heading">Synthetic count task</Title3>
            <div className="task-fields">
              <Field label="Count to">
                <Input
                  type="number"
                  min={1}
                  max={10_000}
                  value={countTo}
                  onChange={(_, data) => setCountTo(data.value)}
                  disabled={!backendReady || taskActive}
                />
              </Field>
              <Field label="Interval in milliseconds">
                <Input
                  type="number"
                  min={10}
                  max={1_000}
                  value={intervalMs}
                  onChange={(_, data) => setIntervalMs(data.value)}
                  disabled={!backendReady || taskActive}
                />
              </Field>
            </div>
            <div className="task-actions">
              <Button
                appearance="primary"
                onClick={() => void beginCount()}
                disabled={!backendReady || taskActive}
              >
                Start count
              </Button>
              <Button
                onClick={() => void requestCancel()}
                disabled={!taskActive || task?.state === "Cancelling"}
              >
                Cancel task
              </Button>
              <Button
                onClick={() => void requestRecovery()}
                disabled={backend?.state !== "Faulted"}
              >
                Recover backend
              </Button>
            </div>

            {task && (
              <div className="task-result">
                <Text weight="semibold">Task: {task.state}</Text>
                {task.progress && (
                  <>
                    <ProgressBar
                      aria-label="Count progress"
                      value={task.progress.current / task.progress.target}
                    />
                    <Text>
                      {task.progress.current} of {task.progress.target}
                    </Text>
                  </>
                )}
                {isTerminal(task) && (
                  <div role="status" aria-live="polite" aria-atomic="true">
                    <Text>
                      Terminal state: {task.state}. Trace: {task.traceId}.
                    </Text>
                    {task.error && (
                      <Text>
                        {task.error.code}: {task.error.message}
                      </Text>
                    )}
                  </div>
                )}
              </div>
            )}
          </section>

          <section className="echo-panel" aria-labelledby="echo-heading">
            <Title3 id="echo-heading">Unicode echo regression</Title3>
            <Field label="Unicode text">
              <Input
                value={text}
                onChange={(_, data) => setText(data.value)}
                disabled={!backendReady || echoBusy || taskActive}
              />
            </Field>
            <Button
              onClick={() => void submitEcho()}
              disabled={!backendReady || echoBusy || taskActive}
            >
              {echoBusy ? "Echoing…" : "Echo"}
            </Button>

            {echoResult && (
              <section className="result" aria-live="polite">
                <Text weight="semibold">Echo result</Text>
                <output>{echoResult}</output>
              </section>
            )}
          </section>

          {error && (
            <div role="alert" className="error">
              <Text weight="semibold">
                {error.code}: {error.message}
              </Text>
              <Text>Trace: {error.traceId}</Text>
            </div>
          )}
        </Card>
      </main>
    </FluentProvider>
  );
}

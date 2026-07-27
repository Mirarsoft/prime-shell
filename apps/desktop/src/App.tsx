import { useEffect, useRef, useState } from "react";
import {
  Button,
  Card,
  Field,
  FluentProvider,
  Input,
  Spinner,
  Text,
  Title1,
  webDarkTheme,
  webLightTheme,
} from "@fluentui/react-components";
import {
  echoText,
  getBackendStatus,
  getRuntimeProbeConfig,
  toSafeError,
  writeRuntimeEvidence,
} from "./backend";
import "./app.css";

const RUNTIME_PROBE_TEXT = "Hello — مرحبا — こんにちは 👋";

function preferredTheme() {
  return window.matchMedia("(prefers-color-scheme: dark)").matches
    ? webDarkTheme
    : webLightTheme;
}

export default function App() {
  const [ready, setReady] = useState(false);
  const [checking, setChecking] = useState(true);
  const [text, setText] = useState("Hello — مرحبا — こんにちは 👋");
  const [result, setResult] = useState("");
  const [error, setError] = useState("");
  const [busy, setBusy] = useState(false);
  const runtimeProbeStarted = useRef(false);
  const cspViolations = useRef<string[]>([]);

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
    void getBackendStatus()
      .then((status) => setReady(status.ready))
      .catch((reason: unknown) => setError(toSafeError(reason).message))
      .finally(() => setChecking(false));
  }, []);

  async function submit() {
    setBusy(true);
    setError("");
    setResult("");
    try {
      const response = await echoText(text);
      setResult(response.text);
    } catch (reason) {
      setError(toSafeError(reason).message);
    } finally {
      setBusy(false);
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

      const status = await getBackendStatus();
      let unicodeResult = "";
      let safeError = "";

      setText(RUNTIME_PROBE_TEXT);
      setError("");
      setResult("");
      if (status.ready) {
        const response = await echoText(RUNTIME_PROBE_TEXT);
        unicodeResult = response.text;
        setResult(response.text);
      }

      try {
        await echoText("x".repeat(262_145));
      } catch (reason) {
        const parsed = toSafeError(reason);
        safeError = `${parsed.code}: ${parsed.message}`;
        setError(parsed.message);
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
        backendReady: status.ready,
        backendVersion: status.backendVersion,
        unicodeInput: RUNTIME_PROBE_TEXT,
        unicodeOutput: unicodeResult,
        unicodeExactMatch: unicodeResult === RUNTIME_PROBE_TEXT,
        renderedText: document.body.innerText,
        safeErrorPath: safeError,
        evidencePath: probe.evidencePath,
      });
    })();
  }, []);

  return (
    <FluentProvider theme={preferredTheme()}>
      <main className="app">
        <Card className="card">
          <header>
            <Title1>Packaged Unicode Echo</Title1>
            <div className="status" aria-live="polite">
              {checking ? (
                <Spinner size="tiny" label="Checking backend" />
              ) : (
                <Text weight="semibold">
                  Backend: {ready ? "Ready" : "Unavailable"}
                </Text>
              )}
            </div>
          </header>

          <Field label="Unicode text">
            <Input
              value={text}
              onChange={(_, data) => setText(data.value)}
              disabled={!ready || busy}
            />
          </Field>

          <Button
            appearance="primary"
            onClick={() => void submit()}
            disabled={!ready || busy}
          >
            {busy ? "Echoing…" : "Echo"}
          </Button>

          {result && (
            <section className="result" aria-live="polite">
              <Text weight="semibold">Result</Text>
              <output>{result}</output>
            </section>
          )}

          {error && (
            <Text role="alert" className="error">
              {error}
            </Text>
          )}
        </Card>
      </main>
    </FluentProvider>
  );
}

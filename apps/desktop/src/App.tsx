import { useEffect, useState } from "react";
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
import { echoText, getBackendStatus, toSafeError } from "./backend";
import "./app.css";

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


import { act, cleanup, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import App from "./App";

type MockChannel = {
  onmessage: (value: unknown) => void;
};

const { invoke, channels } = vi.hoisted(() => ({
  invoke: vi.fn(),
  channels: [] as MockChannel[],
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke,
  Channel: class {
    onmessage = () => undefined;

    constructor() {
      channels.push(this);
    }
  },
}));

const readyStatus = {
  state: "Ready",
  backendVersion: "0.1.0",
  restartCount: 0,
  circuitOpen: false,
  activeTaskId: null,
  lastError: null,
};

const acceptedTask = {
  taskId: "task-1",
  traceId: "trace-task-1",
  operation: "spike.count",
  state: "Running",
  sequence: 0,
  progress: null,
  result: null,
  error: null,
};

function standardInvoke(command: string) {
  if (command === "backend_status") {
    return Promise.resolve(readyStatus);
  }
  if (command === "runtime_probe_config") {
    return Promise.resolve({ enabled: false, evidencePath: null });
  }
  return Promise.reject(new Error(`Unexpected command: ${command}`));
}

beforeEach(() => {
  invoke.mockReset();
  channels.splice(0);
  window.matchMedia = vi.fn().mockReturnValue({
    matches: false,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
  });
});

afterEach(cleanup);

describe("WP02 lifecycle UI", () => {
  it("renders the real Unicode echo result", async () => {
    invoke.mockImplementation((command: string) => {
      if (command === "echo_text") {
        return Promise.resolve({ text: "مرحبا 👋", traceId: "trace-1" });
      }
      return standardInvoke(command);
    });

    render(<App />);
    await screen.findByText("Backend: Ready");

    const input = screen.getByLabelText("Unicode text");
    await userEvent.clear(input);
    await userEvent.type(input, "مرحبا 👋");
    await userEvent.click(screen.getByRole("button", { name: "Echo" }));

    expect(await screen.findByText("مرحبا 👋")).toBeInTheDocument();
    expect(invoke).toHaveBeenLastCalledWith("echo_text", {
      text: "مرحبا 👋",
    });
  });

  it("starts count, shows bounded progress, and announces one terminal", async () => {
    invoke.mockImplementation((command: string) => {
      if (command === "start_count") {
        return Promise.resolve(acceptedTask);
      }
      return standardInvoke(command);
    });

    render(<App />);
    await screen.findByText("Backend: Ready");
    await userEvent.click(screen.getByRole("button", { name: "Start count" }));

    await screen.findByText("Task: Running");
    expect(channels).toHaveLength(1);
    expect(invoke).toHaveBeenCalledWith(
      "start_count",
      expect.objectContaining({
        countTo: 20,
        intervalMs: 100,
        timeoutMs: 30_000,
        onEvent: channels[0],
      }),
    );

    act(() => {
      channels[0].onmessage({
        ...acceptedTask,
        sequence: 1,
        progress: { current: 4, target: 20 },
      });
    });
    expect(screen.getByLabelText("Count progress")).toBeInTheDocument();
    expect(screen.getByText("4 of 20")).toBeInTheDocument();

    act(() => {
      channels[0].onmessage({
        ...acceptedTask,
        state: "Succeeded",
        sequence: 2,
        progress: { current: 20, target: 20 },
        result: { count: 20 },
      });
    });
    const terminal = await screen.findByRole("status");
    expect(terminal).toHaveTextContent("Terminal state: Succeeded");
    expect(terminal).toHaveTextContent("Trace: trace-task-1");
  });

  it("cancels only the active task and renders the terminal safe error", async () => {
    invoke.mockImplementation((command: string) => {
      if (command === "start_count") {
        return Promise.resolve(acceptedTask);
      }
      if (command === "cancel_task") {
        return Promise.resolve({ taskId: "task-1", accepted: true });
      }
      return standardInvoke(command);
    });

    render(<App />);
    await screen.findByText("Backend: Ready");
    await userEvent.click(screen.getByRole("button", { name: "Start count" }));
    await userEvent.click(
      await screen.findByRole("button", { name: "Cancel task" }),
    );
    expect(invoke).toHaveBeenCalledWith("cancel_task", { taskId: "task-1" });
    expect(await screen.findByText("Task: Cancelling")).toBeInTheDocument();

    act(() => {
      channels[0].onmessage({
        ...acceptedTask,
        state: "Cancelled",
        sequence: 1,
        error: {
          code: "CANCELLED",
          message: "The synthetic task was cancelled.",
          traceId: "trace-task-1",
        },
      });
    });
    const terminal = await screen.findByRole("status");
    expect(terminal).toHaveTextContent("CANCELLED");
    expect(terminal).toHaveTextContent(
      "The synthetic task was cancelled.",
    );
  });

  it("offers explicit recovery only for a faulted backend", async () => {
    const faulted = {
      ...readyStatus,
      state: "Faulted",
      circuitOpen: true,
      lastError: {
        code: "BACKEND_CRASHED",
        message: "The packaged backend stopped unexpectedly.",
        traceId: "backend-crash",
      },
    };
    invoke.mockImplementation((command: string) => {
      if (command === "backend_status") {
        return Promise.resolve(faulted);
      }
      if (command === "recover_backend") {
        return Promise.resolve(readyStatus);
      }
      if (command === "runtime_probe_config") {
        return Promise.resolve({ enabled: false, evidencePath: null });
      }
      return Promise.reject(new Error(`Unexpected command: ${command}`));
    });

    render(<App />);
    await screen.findByText("Backend: Faulted");
    const recovery = screen.getByRole("button", { name: "Recover backend" });
    expect(recovery).toBeEnabled();
    await userEvent.click(recovery);
    expect(await screen.findByText("Backend: Ready")).toBeInTheDocument();
  });

  it("shows bounded safe code, message, and trace data", async () => {
    invoke.mockImplementation((command: string) => {
      if (command === "echo_text") {
        return Promise.reject({
          code: "BACKEND_UNAVAILABLE",
          message: "Backend unavailable.",
          traceId: "trace-2",
        });
      }
      return standardInvoke(command);
    });

    render(<App />);
    await screen.findByText("Backend: Ready");
    await userEvent.click(screen.getByRole("button", { name: "Echo" }));

    await waitFor(() => {
      const alert = screen.getByRole("alert");
      expect(alert).toHaveTextContent("BACKEND_UNAVAILABLE");
      expect(alert).toHaveTextContent("Backend unavailable.");
      expect(alert).toHaveTextContent("Trace: trace-2");
    });
  });
});

import { cleanup, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import App from "./App";

const { invoke } = vi.hoisted(() => ({ invoke: vi.fn() }));

vi.mock("@tauri-apps/api/core", () => ({
  invoke,
  isTauri: () => false,
}));

beforeEach(() => {
  invoke.mockReset();
  window.matchMedia = vi.fn().mockReturnValue({
    matches: false,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
  });
});

afterEach(cleanup);

describe("Unicode echo UI", () => {
  it("renders the real command result", async () => {
    invoke
      .mockResolvedValueOnce({ ready: true, backendVersion: "0.1.0" })
      .mockResolvedValueOnce({ text: "مرحبا 👋", traceId: "trace-1" });

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

  it("shows a bounded safe error", async () => {
    invoke
      .mockResolvedValueOnce({ ready: true, backendVersion: "0.1.0" })
      .mockRejectedValueOnce({
        code: "BACKEND_UNAVAILABLE",
        message: "Backend unavailable.",
        traceId: "trace-2",
      });

    render(<App />);
    await screen.findByText("Backend: Ready");
    await userEvent.click(screen.getByRole("button", { name: "Echo" }));

    await waitFor(() =>
      expect(screen.getByRole("alert")).toHaveTextContent(
        "Backend unavailable.",
      ),
    );
  });
});

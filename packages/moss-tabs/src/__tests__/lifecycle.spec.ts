import { describe, expect, test, vi } from "vitest";

import { CompositeDisposable, Disposable, MutableDisposable } from "../lifecycle";

describe("lifecycle", () => {
  test("mutable disposable", () => {
    const mutableDisposable = new MutableDisposable();

    let disposed = 0;

    const disposable = () => ({
      dispose: () => {
        disposed++;
      },
    });

    mutableDisposable.value = disposable();
    expect(disposed).toBe(0);

    mutableDisposable.value = disposable();
    expect(disposed).toBe(1);

    mutableDisposable.dispose();
    expect(disposed).toBe(2);

    mutableDisposable.dispose();
  });

  test("composite disposable", () => {
    const d1 = {
      dispose: vi.fn(),
    };
    const d2 = {
      dispose: vi.fn(),
    };
    const d3 = {
      dispose: vi.fn(),
    };
    const d4 = {
      dispose: vi.fn(),
    };

    const cut = new CompositeDisposable(d1, d2);
    cut.addDisposables(d3, d4);

    cut.dispose();

    expect(d1.dispose).toHaveBeenCalledTimes(1);
    expect(d2.dispose).toHaveBeenCalledTimes(1);
    expect(d3.dispose).toHaveBeenCalledTimes(1);
    expect(d4.dispose).toHaveBeenCalledTimes(1);
  });

  test("that isDisposed=true once CompositeDisposable is disposed", () => {
    class Test extends CompositeDisposable {
      checkIsDisposed(): boolean {
        return this.isDisposed;
      }
    }

    const cut = new Test();

    expect(cut.checkIsDisposed()).toBeFalsy();

    cut.dispose();

    expect(cut.checkIsDisposed()).toBeTruthy();
  });

  test("Disposable.from(...)", () => {
    const func = vi.fn();

    const disposable = Disposable.from(func);

    expect(func).not.toHaveBeenCalled();

    disposable.dispose();

    expect(func).toHaveBeenCalledTimes(1);
  });
});

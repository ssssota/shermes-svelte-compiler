import { compile, compileModule } from "svelte/compiler";
import type { CompileResult } from "svelte/compiler";

declare global {
  namespace globalThis {
    var svelteCompile: (source: string) => CompileResult;
    var svelteCompileModule: (source: string) => CompileResult;
  }
}

globalThis.svelteCompile = (source: string) => {
  const result = compile(source, {});
  return result;
};
globalThis.svelteCompileModule = (source: string) => {
  const result = compileModule(source, {});
  return result;
};

// polyfill: Hermes does not support structuredClone yet.
globalThis.structuredClone ??= (obj: any) => {
  return JSON.parse(JSON.stringify(obj));
};

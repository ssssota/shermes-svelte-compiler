#include <stdlib.h>
#include <hermes/VM/static_h.h>
#include <hermes/hermes.h>

// Declaration for the `svelte` unit created by Static Hermes.
// This will come from `svelte.o`
extern "C" SHUnit sh_export_svelte;

// Free memory allocated by this library
extern "C" void free_string(char *ptr) {
  delete[] ptr;
}

// Shared runtime instances
static SHRuntime *s_shRuntime = nullptr;
static facebook::hermes::HermesRuntime *s_hermes = nullptr;

static void ensureRuntimeInitialized() {
  if (s_shRuntime == nullptr) {
    s_shRuntime = _sh_init(0, nullptr);
    s_hermes = _sh_get_hermes_runtime(s_shRuntime);
    if (!_sh_initialize_units(s_shRuntime, 1, &sh_export_svelte)) {
      abort();
    }
  }
}

typedef struct {
  char *js;
  char *css;
} SvelteCompileResult;
extern "C" void free_svelte_compile_result(SvelteCompileResult *result) {
  delete[] result->js;
  delete[] result->css;
}

char* get_code(
  facebook::jsi::Runtime& rt,
  const facebook::jsi::Value& val
) {
  if (val.isUndefined() || val.isNull()) {
    return nullptr;
  }
  std::string code = val
    .getObject(rt)
    .getProperty(rt, "code")
    .getString(rt)
    .utf8(rt);
  char* code_str = new char[code.size() + 1];
  strcpy(code_str, code.c_str());
  return code_str;
}

SvelteCompileResult build_result(
  facebook::jsi::Runtime& rt,
  facebook::jsi::Value& val
) {
  facebook::jsi::Object res = val.getObject(rt);
  SvelteCompileResult result = {
    get_code(rt, res.getProperty(rt, "js")),
    get_code(rt, res.getProperty(rt, "css")),
  };
  return result;
}

extern "C" SvelteCompileResult compile(char *in) {
  ensureRuntimeInitialized();

  facebook::jsi::Value val = s_hermes->global()
    .getPropertyAsFunction(*s_hermes, "svelteCompile")
    .call(*s_hermes, std::string(in));

  return build_result(*s_hermes, val);
}

extern "C" SvelteCompileResult compile_module(char *in) {
  ensureRuntimeInitialized();

  facebook::jsi::Value val = s_hermes->global()
    .getPropertyAsFunction(*s_hermes, "svelteCompileModule")
    .call(*s_hermes, std::string(in));

  return build_result(*s_hermes, val);
}

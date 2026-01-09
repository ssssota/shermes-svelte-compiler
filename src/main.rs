use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr::NonNull;

// Declare the C function we will call.
extern "C" {
    fn compile(input: *const c_char) -> SvelteCompileResult;
    fn compile_module(input: *const c_char) -> SvelteCompileResult;
    fn free_string(s: *const c_char);
    fn free_svelte_compile_result(res: *const SvelteCompileResult);
}

#[repr(C)]
struct SvelteCompileResult {
    js: *const c_char,
    css: *const c_char,
}

struct CCharPtr(NonNull<c_char>);
impl Drop for CCharPtr {
    fn drop(&mut self) {
        unsafe { free_string(self.0.as_ptr()) }
    }
}
impl CCharPtr {
    fn new(ptr: *const c_char) -> Option<Self> {
        NonNull::new(ptr as *mut c_char).map(CCharPtr)
    }
}
impl TryFrom<CCharPtr> for String {
    type Error = &'static str;

    fn try_from(c_char_ptr: CCharPtr) -> Result<Self, Self::Error> {
        let s = unsafe { CStr::from_ptr(c_char_ptr.0.as_ptr()) };
        Ok(s.to_string_lossy().into_owned())
    }
}

fn main() {
    let module_input = CString::new("var a = 0;").unwrap();
    let module_output = unsafe {
        let res = compile_module(module_input.as_ptr());
        let js = CStr::from_ptr(res.js).to_string_lossy().into_owned();
        // free_svelte_compile_result(&res);
        js
    };
    println!("MODULE OUTPUT: {}", module_output);

    // Create a C string.
    let input = CString::new(
        r#"
        <script>
          let name = $state('world');
        </script>
        <input bind:value={name}>
        <h1>Hello {name}!</h1>
        <style>
          h1 { color: purple; }
        </style>
        "#,
    )
    .unwrap();

    // Call the C function and convert it to a Rust String.
    let res = unsafe {
        let res = compile(input.as_ptr());
        let js = CStr::from_ptr(res.js).to_string_lossy().into_owned();
        // free_svelte_compile_result(&res);
        js
    };

    // Print it.
    println!("OUTPUT: {}", res);
}

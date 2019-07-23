use wasm_bindgen::prelude::*;

// the path to a file, just for clarity here
type Path = String;

// some zokrates source, just for clarity
type Source = String;

#[wasm_bindgen]
extern "C" {
    fn resolve(p: Path) -> Source;
}

// This is a mock of the zokrates API with some changes.
// Using Strings not buffers
// not using references
mod zokrates {
    use super::*;

    // we change this from a function pointer to something that implements FnOnce as we need to support closures, see further down
    // a resolver basically takes a path and returns the source at that path. Again, simplified here.
    pub type Resolve = dyn FnOnce(Path) -> Result<Source, ()>;

    // just mocking here, this is actually a struct in real life
    pub type Prog = String;

    pub fn compile(
        _: String,
        _: Option<String>,
        resolve_option: Option<Box<Resolve>>,
    ) -> Result<Prog, ()> {
        match resolve_option {
            Some(resolve) => {
                // in the process of compiling, we may have to call resolve
                let _ = resolve("path/to/dep".to_string());
                // do some other things with the result...
                // ...and eventually we return a program
                Ok(String::from("<some program>"))
            }
            None => unreachable!(), // we assume that we have a resolver here so crash if we don't
        }
    }
}

#[wasm_bindgen]
/// this is the function that we expose to the user, and to which the user can pass their own resolver
/// they pass their source and a resolver like `s.compile("def main() -> (): return", resolve)`
pub fn compile(source: JsValue) -> JsValue {
    // Here we create a rust closure which calls the passed resolver when called
    // it is assumed that `resolver` is actually a function with one variable, it is the responsibility of the caller (say remix) to enforce that
    let resolve_closure = |a: Path| -> Result<Source, ()> { Ok(resolve(a)) };

    // we call the zokrates compile function with our closure
    zokrates::compile(
        source.as_string().unwrap(),
        Some("path/to/main".to_string()),
        Some(Box::new(resolve_closure)),
    )
    .unwrap()
    .into()
}

// generic things, were in the template I used for this
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

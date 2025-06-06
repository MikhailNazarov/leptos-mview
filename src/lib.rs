/*!
An alternative `view!` macro for [Leptos](https://github.com/leptos-rs/leptos/tree/main) inspired by [maud](https://maud.lambda.xyz/).

# Example

A little preview of the syntax:

```
use leptos::prelude::*;
use leptos_mview::mview;

#[component]
fn MyComponent() -> impl IntoView {
    let (value, set_value) = signal(String::new());
    let red_input = move || value().len() % 2 == 0;

    mview! {
        h1.title("A great website")
        br;

        input
            type="text"
            data-index=0
            class:red={red_input}
            prop:{value}
            on:change={move |ev| {
                set_value(event_target_value(&ev))
            }};

        Show
            when=[!value().is_empty()]
            fallback=[mview! { "..." }]
        (
            Await
                future={fetch_from_db(value())}
                blocking
            |db_info| (
                p("Things found: " strong({*db_info}) "!")
                p("Is bad: " f["{}", red_input()])
            )
        )
    }
}

async fn fetch_from_db(data: String) -> usize { data.len() }
```

<details>
<summary> Explanation of the example: </summary>

```
use leptos::prelude::*;
use leptos_mview::mview;

#[component]
fn MyComponent() -> impl IntoView {
    let (value, set_value) = signal(String::new());
    let red_input = move || value().len() % 2 == 0;

    mview! {
        // specify tags and attributes, children go in parentheses.
        // classes (and ids) can be added like CSS selectors.
        // same as `h1 class="title"`
        h1.title("A great website")
        // elements with no children end with a semi-colon
        br;

        input
            type="text"
            data-index=0 // kebab-cased identifiers supported
            class:red={red_input} // non-literal values must be wrapped in braces
            prop:{value} // shorthand! same as `prop:value={value}`
            on:change={move |ev| { // event handlers same as leptos
                set_value(event_target_value(&ev))
            }};

        Show
            // values wrapped in brackets `[body]` are expanded to `{move || body}`
            when=[!value().is_empty()] // `{move || !value().is_empty()}`
            fallback=[mview! { "..." }] // `{move || mview! { "..." }}`
        ( // I recommend placing children like this when attributes are multi-line
            Await
                future={fetch_from_db(value())}
                blocking // expanded to `blocking=true`
            // children take arguments with a 'closure'
            // this is very different to `let:db_info` in Leptos!
            |db_info| (
                p("Things found: " strong({*db_info}) "!")
                // bracketed expansion works in children too!
                // this one also has a special prefix to add `format!` into the expansion!
                //    {move || format!("{}", red_input()}
                p("Is bad: " f["{}", red_input()])
            )
        )
    }
}

// fake async function
async fn fetch_from_db(data: String) -> usize { data.len() }
```

</details>

# Purpose

The `view!` macros in Leptos is often the largest part of a component, and can get extremely long when writing complex components. This macro aims to be as **concise** as possible, trying to **minimise unnecessary punctuation/words** and **shorten common patterns**.

# Compatibility

This macro will be compatible with the latest stable release of Leptos. The macro references Leptos items using `::leptos::...`, no items are re-exported from this crate. Therefore, this crate will likely work with any Leptos version if no view-related items are changed.

The below are the versions with which I have tested it to be working. It is likely that the macro works with more versions of Leptos.

| `leptos_mview` version | Compatible `leptos` version |
| ---------------------- | --------------------------- |
| `0.1`                  | `0.5`                       |
| `0.2`                  | `0.5`, `0.6`                |
| `0.3`                  | `0.6`                       |
| `0.4`                  | `0.7`                       |

This crate also has a feature `"nightly"` that enables better proc-macro diagnostics (simply enables the nightly feature in proc-macro-error2. Necessary while [this pr](https://github.com/GnomedDev/proc-macro-error-2/pull/5) is not yet merged).

# Syntax details

## Elements

Elements have the following structure:

1. Element / component tag name / path (`div`, `App`, `component::Codeblock`).
2. Any classes or ids prefixed with a dot `.` or hash `#` respectively.
3. A space-separated list of attributes and directives (`class="primary"`, `on:click={...}`).
4. Children in parens or braces (`("hi")` or `{ "hi!" }`), or a semi-colon for no children (`;`).

Example:
```
# use leptos_mview::mview; use leptos::prelude::*;
# let handle_input = |_| ();
# #[component] fn MyComponent(data: i32, other: &'static str) -> impl IntoView {}
mview! {
    div.primary(strong("hello world"))
    input type="text" on:input={handle_input};
    MyComponent data=3 other="hi";
}
# ;
```

Adding generics is the same as in Leptos: add it directly after the component name, with or without the turbofish.

```
# use leptos::prelude::*; use leptos_mview::mview;
# use core::marker::PhantomData;
#[component]
pub fn GenericComponent<S>(ty: PhantomData<S>) -> impl IntoView {
    std::any::type_name::<S>()
}

#[component]
pub fn App() -> impl IntoView {
    mview! {
        // both with and without turbofish is supported
        GenericComponent::<String> ty={PhantomData};
        GenericComponent<usize> ty={PhantomData};
        GenericComponent<i32> ty={PhantomData};
    }
}
```

Note that due to [Reserving syntax](https://doc.rust-lang.org/edition-guide/rust-2021/reserving-syntax.html), the `#` for ids must have a space before it.

```
# use leptos_mview::mview; use leptos::prelude::*;
mview! {
    nav #primary ("...")
    // not allowed: nav#primary ("...")
}
# ;
```

Classes/ids created with the selector syntax can be mixed with the attribute `class="..."` and directive `class:a-class={signal}` as well.

There is also a special element `!DOCTYPE html;`, equivalent to `<!DOCTYPE html>`.

## Slots

[Slots](https://docs.rs/leptos/latest/leptos/attr.slot.html) ([another example](https://github.com/leptos-rs/leptos/blob/main/examples/slots/src/lib.rs)) are supported by prefixing the struct with `slot:` inside the parent's children.

The name of the parameter in the component function must be the same as the slot's name, in snake case.

Using the slots defined by the [`SlotIf` example linked](https://github.com/leptos-rs/leptos/blob/main/examples/slots/src/lib.rs):
```
use leptos::prelude::*;
use leptos_mview::mview;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let is_even = Signal::derive(move || count() % 2 == 0);
    let is_div5 = Signal::derive(move || count() % 5 == 0);
    let is_div7 = Signal::derive(move || count() % 7 == 0);

    mview! {
        SlotIf cond={is_even} (
            slot:Then ("even")
            slot:ElseIf cond={is_div5} ("divisible by 5")
            slot:ElseIf cond={is_div7} ("divisible by 7")
            slot:Fallback ("odd")
        )
    }
}
# #[slot] struct Then { children: ChildrenFn }
# #[slot] struct ElseIf { #[prop(into)] cond: Signal<bool>, children: ChildrenFn }
# #[slot] struct Fallback { children: ChildrenFn }
#
# #[component]
# fn SlotIf(
#     #[prop(into)] cond: Signal<bool>,
#     then: Then,
#     #[prop(optional)] else_if: Vec<ElseIf>,
#     #[prop(optional)] fallback: Option<Fallback>,
# ) -> impl IntoView {
#     move || {
#         if cond() {
#             (then.children)().into_any()
#         } else if let Some(else_if) = else_if.iter().find(|i| (i.cond)()) {
#             (else_if.children)().into_any()
#         } else if let Some(fallback) = &fallback {
#             (fallback.children)().into_any()
#         } else {
#             ().into_any()
#         }
#     }
# }
```

## Values

There are (currently) 3 main types of values you can pass in:

- **Literals** can be passed in directly to attribute values (like `data=3`, `class="main"`, `checked=true`).
    - However, children do not accept literal numbers or bools - only strings.
        ```compile_fail
        # use leptos_mview::mview;
        // does NOT compile.
        mview! { p("this works " 0 " times: " true) }
        # ;
        ```

- Everything else must be passed in as a **block**, including variables, closures, or expressions.
    ```
    # use leptos_mview::mview; use leptos::prelude::*;
    # let input_type = "text";
    # let handle_input = |_a: i32| ();
    mview! {
        input
            class="main"
            checked=true
            data-index=3
            type={input_type}
            on:input={move |_| handle_input(1)};
    }
    # ;
    ```

    This is not valid:
    ```compile_fail
    # use leptos_mview::mview;
    let input_type = "text";
    // ❌ This is not valid! Wrap input_type in braces.
    mview! { input type=input_type }
    # ;
    ```

- Values wrapped in **brackets** (like `value=[a_bool().to_string()]`) are shortcuts for a block with an empty closure `move || ...` (to `value={move || a_bool().to_string()}`).
    ```rust
    # use leptos::prelude::*; use leptos_mview::mview;
    # let number = || 3;
    mview! {
        Show
            fallback=[()] // common for not wanting a fallback as `|| ()`
            when=[number() % 2 == 0] // `{move || number() % 2 == 0}`
        (
            "number + 1 = " [number() + 1] // works in children too!
        )
    }
    # ;
    ```

    - Note that this always expands to `move || ...`: for any closures that take an argument, use the full closure block instead.
        ```compile_error
        # use leptos_mview::mview;
        # use leptos::logging::log;
        mview! {
            input type="text" on:click=[log!("THIS DOESNT WORK")];
        }
        ```

        Instead:
        ```
        # use leptos_mview::mview; use leptos::prelude::*;
        # use leptos::logging::log;
        mview! {
            input type="text" on:click={|_| log!("THIS WORKS!")};
        }
        # ;
        ```

The bracketed values can also have some special prefixes for even more common shortcuts!
- Currently, the only one is `f` - e.g. `f["{:.2}", stuff()]`. Adding an `f` will add `format!` into the closure. This is equivalent to `[format!("{:.2}", stuff())]` or `{move || format!("{:.2}", stuff())}`.

## Attributes

### Key-value attributes

Most attributes are `key=value` pairs. The `value` follows the rules from above. The `key` has a few variations:

- Standard identifier: identifiers like `type`, `an_attribute`, `class`, `id` etc are valid keys.
- Kebab-case identifier: identifiers can be kebab-cased, like `data-value`, `an-attribute`.
    - NOTE: on HTML elements, this will be put on the element as is: `div data-index="0";` becomes `<div data-index="0"></div>`. **On components**, hyphens are converted to underscores then passed into the component builder.

        For example, this component:
        ```ignore
        #[component]
        fn Something(some_attribute: i32) -> impl IntoView { ... }
        ```

        Can be used elsewhere like this:
        ```
        # use leptos::prelude::*; use leptos_mview::mview;
        # #[component] fn Something(some_attribute: i32) -> impl IntoView {}
        mview! { Something some-attribute=5; }
        # ;
        ```

        And the `some-attribute` will be passed in to the `some_attribute` argument.

- Attribute shorthand: if the name of the attribute and value are the same, e.g. `class={class}`, you can replace this with `{class}` to mean the same thing.
    ```
    # use leptos_mview::mview; use leptos::prelude::*;
    let class = "these are classes";
    let id = "primary";
    mview! {
        div {class} {id} ("this has 3 classes and id='primary'")
    }
    # ;
    ```

    See also: [kebab-case identifiers with attribute shorthand](#kebab-case-identifiers-with-attribute-shorthand)

Note that the special `node_ref` or `ref` or `_ref` or `ref_` attribute in Leptos to bind the element to a variable is just `ref={variable}` in here.

### Boolean attributes

Another shortcut is that boolean attributes can be written without adding `=true`. Watch out though! `checked` is **very different** to `{checked}`.
```
# use leptos::prelude::*; use leptos_mview::mview;
// recommend usually adding #[prop(optional)] to all these
#[component]
fn LotsOfFlags(wide: bool, tall: bool, red: bool, curvy: bool, count: i32) -> impl IntoView {}

mview! { LotsOfFlags wide tall red=false curvy count=3; }
# ;
// same as...
mview! { LotsOfFlags wide=true tall=true red=false curvy=true count=3; }
# ;
```

See also: [boolean attributes on HTML elements](#boolean-attributes-on-html-elements)

### Directives

Some special attributes (distinguished by the `:`) called **directives** have special functionality. All have the same behaviour as Leptos. These include:
- `class:class-name=[when to show]`
- `style:style-key=[style value]`
- `on:event={move |ev| event handler}`
- `prop:property-name={signal}`
- `attr:name={value}`
- `clone:ident_to_clone`
- `use:directive_name` or `use:directive_name={params}`
- `bind:checked={rwsignal}` or `bind:value={(getter, setter)}`

All of these directives except `clone` also support the attribute shorthand:

```
# use leptos::prelude::*; use leptos_mview::mview;
let color = RwSignal::new("red".to_string());
let disabled = false;
mview! {
    div style:{color} class:{disabled};
}
# ;
```

The `class` and `style` directives also support using string literals, for more complicated names. Make sure the string for `class:` doesn't have spaces, or it will panic!

```
# use leptos::prelude::*; use leptos_mview::mview;
let yes = move || true;
mview! {
    div class:"complex-[class]-name"={yes}
        style:"doesn't-exist"="white";
}
# ;
```

Note that the `use:` directive automatically calls `.into()` on its argument, consistent with behaviour from Leptos.

## Children

You may have noticed that the `let:data` prop was missing from the previous section on directive attributes!

This is replaced with a closure right before the children block. This way, you can pass in multiple arguments to the children more easily.

```
# use leptos::prelude::*; use leptos_mview::mview;
# leptos::task::Executor::init_futures_executor().unwrap();
mview! {
    Await
        future={async { 3 }}
    |monkeys| (
        p({*monkeys} " little monkeys, jumping on the bed.")
    )
}
# ;
```

Note that you will usually need to add a `*` before the data you are using. If you forget that, rust-analyser will tell you to dereference here: `*{monkeys}`. This is obviously invalid - put it inside the braces.

Children can be wrapped in either braces or parentheses, whichever you prefer.

```
# use leptos::prelude::*; use leptos_mview::mview;
mview! {
    p {
        "my " strong("bold") " and " em("fancy") " text."
    }
}
# ;
```

Summary from the previous section on values in case you missed it: children can be literal strings (not bools or numbers!), blocks with Rust code inside (`{*monkeys}`), or the closure shorthand `[number() + 1]`.

Children with closures are also supported on slots.

# Extra details

## Kebab-case identifiers with attribute shorthand

If an attribute shorthand has hyphens:
- On components, both the key and value will be converted to underscores.
    ```
    # use leptos::prelude::*; use leptos_mview::mview;
    # #[component] fn Something(some_attribute: i32) -> impl IntoView {}
    let some_attribute = 5;
    mview! { Something {some-attribute}; }
    # ;
    // same as...
    mview! { Something {some_attribute}; }
    # ;
    // same as...
    mview! { Something some_attribute={some_attribute}; }
    # ;
    ```

- On HTML elements, the key will keep hyphens, but the value will be turned into an identifier with underscores.
    ```
    # use leptos_mview::mview; use leptos::prelude::*;
    let aria_label = "a good label";
    mview! { input {aria-label}; }
    # ;
    // same as...
    mview! { input aria-label={aria_label}; }
    # ;
    ```

## Boolean attributes on HTML elements

Note the behaviour from Leptos: setting an HTML attribute to true adds the attribute with no value associated.
```
# use leptos::prelude::*;
view! { <input type="checkbox" checked=true data-smth=true not-here=false /> }
# ;
```
Becomes `<input type="checkbox" checked data-smth />`, NOT `checked="true"` or `data-smth="true"` or `not-here="false"`.

To have the attribute have a value of the string "true" or "false", use `.to_string()` on the bool. Make sure that it's in a closure if you're working with signals too.
```
# use leptos::prelude::*;
# use leptos_mview::mview;
let boolean_signal = RwSignal::new(true);
mview! { input type="checkbox" checked=[boolean_signal().to_string()]; }
# ;
// or, if you prefer
mview! { input type="checkbox" checked=f["{}", boolean_signal()]; }
# ;
```

# Contributing

Please feel free to make a PR/issue if you have feature ideas/bugs to report/feedback :)

 */

// note: to transfer above to README.md, install `cargo-rdme` and run
// `cargo rdme`
// Some bits are slightly broken, fix up stray `compile_error`/
// `ignore`, missing `rust` annotations and remove `#` lines.

pub use leptos_mview_macro::mview;

/// Not for public use. Do not implement anything on this.
#[doc(hidden)]
pub struct MissingValueAfterEq;

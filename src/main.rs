mod error_views;
mod templates;

use perseus::{plugins::Plugins, prelude::*};

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .error_views(error_views::get_error_views())
        .index_view(|cx| {
            sycamore::view! { cx,
                html(class = "light") {
                    head {
                        meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        link(rel = "stylesheet", href = ".perseus/static/tailwind.css")
                        link(rel = "stylesheet", href = ".perseus/static/styles/style.css")
                    }
                    body(class = "bg-white dark:bg-neutral-900") {
                        PerseusRoot()
                    }
                }
            }
        })
        .plugins(Plugins::new().plugin(
            perseus_tailwind::get_tailwind_plugin,
            perseus_tailwind::TailwindOptions {
                in_file: "/tailwind.css".into(),
                // Don't put this in /static, it will trigger build loops.
                // Put this in /dist and use a static alias instead.
                out_file: "static/tailwind.css".into(),
            },
        ))
        .static_alias("/tailwind.css", "static/tailwind.css")
}

/// A universal representation of error messages that can occur in the app. This
/// is fully compatible with the Perseus state generation system.
#[cfg(engine)]
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
struct Error(#[from] Box<dyn std::error::Error + Send + Sync>);
// This is not designed to be used as a 'proper' `From<E>` implementation, it's
// designed to be used in `some_function().map_err(Error::from)?`, which allows
// converting any error type straight into this for convenience.
//
// Perseus requires you to be explicit about your errors, mainly to avoid
// potentially leaking sensitive details to clients, which could be caused by
// this sort of blind conversion. Hence, (and due to internal Rust constraints
// on `?`), Perseus deliberatly avoids exposing this kind of function itself.
#[cfg(engine)]
impl Error {
    #[inline]
    fn from<E: std::error::Error + Send + Sync + 'static>(value: E) -> Self {
        Error(value.into())
    }
}
#[cfg(engine)]
impl From<String> for Error {
    fn from(msg: String) -> Self {
        let boxed: Box<dyn std::error::Error + Send + Sync> = msg.into();
        boxed.into()
    }
}

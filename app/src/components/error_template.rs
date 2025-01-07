use http::status::StatusCode;
use leptos::{
    html::{a, div, h1},
    prelude::*,
    svg::{path, svg},
};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
pub fn component(outside_errors: Option<Errors>, errors: Option<RwSignal<Errors>>) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => RwSignal::new(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    // Get Errors from Signal
    let errors = errors.get_untracked();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    println!("Errors: {errors:#?}");

    // Only the response code for the first error is actually sent from the ssr
    // this may be customized by the specific application
    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());
        }
    }

    div().class("grid place-content-center px-4 h-full antialiased").child((
        h1().class("mb-6 text-center").child(if errors.len() > 1 { "Errors" } else { "Error" }),
        For(
            ForProps::builder()
                .each(move || errors.clone().into_iter().enumerate())
                .key(|(index, _error)| *index)
                .children(|error| {
                    let error_string = error.1.to_string();
                    let error_code = error.1.status_code();

                    div().child((
                        h1().class("text-xl tracking-widest text-gray-400 uppercase").child(
                            format!("{}| {}", error_code, error_string),
                        ),
                        a()
                            .href("/")
                            .class("flex gap-1 justify-center items-center mt-6 text-center duration-200 hover:text-[#68b5fc]")
                            .child((
                                svg().attr("width", "1.1em").attr("height", "1.1em").attr("viewBox", "0 0 24 24").attr("fill", "currentColor").attr("role", "graphics-symbol").attr("data-hk", "0-0-0-98").child(
                                    path().attr("d", "M21 11H6.414l5.293-5.293-1.414-1.414L2.586 12l7.707 7.707 1.414-1.414L6.414 13H21z"),
                                ),
                                "Go back home",
                            )),
                    ))
                }).build(),
        ),
    ))
}

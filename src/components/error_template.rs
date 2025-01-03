use http::status::StatusCode;
use leptos::prelude::*;
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
#[component]
pub fn Component(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
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

    view! {
        <div class="grid place-content-center px-4 h-full antialiased">
            <h1 class="mb-6 text-center">{if errors.len() > 1 { "Errors" } else { "Error" }}</h1>
            <For
                // a function that returns the items we're iterating over; a signal is fine
                each=move || { errors.clone().into_iter().enumerate() }
                // a unique key for each item as a reference
                key=|(index, _error)| *index
                // renders each item to a view
                children=move |error| {
                    let error_string = error.1.to_string();
                    let error_code = error.1.status_code();
                    view! {
                        <h1 class="text-xl tracking-widest text-gray-400 uppercase">
                            {error_code.to_string()}| {error_string}
                        </h1>
                        <a
                            href="/"
                            class="flex gap-1 justify-center items-center mt-6 text-center duration-200 hover:text-[#68b5fc]"
                        >
                            <svg
                                width="1.1em"
                                height="1.1em"
                                viewBox="0 0 24 24"
                                fill="currentColor"
                                role="graphics-symbol"
                                data-hk="0-0-0-98"
                            >
                                <path d="M21 11H6.414l5.293-5.293-1.414-1.414L2.586 12l7.707 7.707 1.414-1.414L6.414 13H21z"></path>
                            </svg>
                            Go back home
                        </a>
                    }
                }
            />
        </div>
    }
}

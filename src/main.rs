use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div class="min-h-screen bg-gray-100 p-8">
            <div class="max-w-md mx-auto bg-white rounded-lg shadow-md p-6">
                <h1 class="text-3xl font-bold text-center mb-6">"Rust UI App"</h1>
                <div class="text-center">
                    <p class="text-xl mb-4">"Count: " {move || count.get()}</p>
                    <button 
                        class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 mr-2"
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        "+"
                    </button>
                    <button 
                        class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 mr-2"
                        on:click=move |_| set_count.update(|n| *n -= 1)
                    >
                        "-"
                    </button>
                    <button 
                        class="px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600"
                        on:click=move |_| set_count.set(0)
                    >
                        "Reset"
                    </button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> });
}
use leptos::*;
use leptos_meta::*;
use leptos_remix_icon::Icon;
use leptos_router::*;
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (_count, _set_count) = create_signal(0);

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">" Leptos + Tailwind + Remix Icon"</h2>
            <p class="px-10 pb-10 text-left">
                "Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."
            </p>



             <div class="flex items-center justify-center">
             <Icon class="text-2xl my-8" icon="github-fill" size="10x" />
             <Icon class="text-2xl my-8" icon="github-fill" size="9x" />
             <Icon class="text-2xl my-8" icon="github-fill" size="8x" />
             <Icon class="text-2xl my-8" icon="github-fill" size="8x" />
             <Icon class="text-2xl my-8" icon="github-fill" size="7x" />
             <Icon class="text-2xl my-8" icon="github-fill" size="6x" />
             <Icon class="text-2xl my-8" icon="github-fill" size="5x" />
             <Icon class="text-2xl my-8" icon="github-fill" size="4x" />
         <Icon class="text-2xl block" icon="github-fill" size="3x"/>
             <Icon class="text-2xl my-8" icon="github-fill" size="2x" />
             <Icon class="text-2xl my-8" icon="github-fill"  />


            </div>
        </div>
    }
}

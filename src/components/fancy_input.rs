use leptos::prelude::*;
use leptos::text_prop::TextProp;

#[component]
pub fn FancyInput(
    #[prop(into)] label: TextProp,
    #[prop(optional, into)] password: Signal<bool>,
    #[prop(optional)] required: bool,
    #[prop(optional)] value: RwSignal<String>,
) -> impl IntoView {
    view! {
        <div class="relative mt-12 mb-3">
            <input
                class="peer text-lg w-full relative p-1 z-20 border-none outline-none bg-transparent text-white"
                type={move || if password.get() { "password" } else { "text" }}
                required={required}
                placeholder=""
                bind:value={value}
            />
            <span
                class="absolute left-0 text-lg transition-all duration-500
                    text-lg peer-[:not(:placeholder-shown)]:text-base peer-focus:text-base
                    text-black peer-[:not(:placeholder-shown)]:text-neutral-700 peer-focus:text-neutral-700;
                    peer-[:not(:placeholder-shown)]:translate-y-[-30px] peer-focus:translate-y-[-30px]" 
            >
                {label.get()}
            </span>
            <div
                class="w-full h-[2px] rounded-md bg-accent-light absolute bottom-0 left-0
                    transition-all duration-500 peer-[:not(:placeholder-shown)]:h-10 peer-focus:h-10"
            ></div>
        </div>
    }
}

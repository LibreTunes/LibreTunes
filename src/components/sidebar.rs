use leptos::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="sidebar-container">
            <Top />
            <div class="sidebar-header">
                <h1>LibreTunes</h1>
            </div>
            
        </div>
    }
}
#[component]
pub fn Top() -> impl IntoView {
    view! {
        <div class="sidebar-top-container">
            <h1>Hello</h1>
        </div>
    }
}

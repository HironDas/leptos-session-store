use leptos::prelude::*;
use leptos_icons::Icon;
/// Documentation for [`Login`]
#[component]
pub fn Login() -> impl IntoView {
    

    view! {
        <div class="flex justify-center items-center h-screen">
        <div class="w-100 h-100 bg-white rounded-lg shadow-lg flex justify-center p-6">
            "Login Page"
            <Icon icon={icondata::AiUserOutlined} width="30" height="30"/>
        </div>
        </div>
    }
}

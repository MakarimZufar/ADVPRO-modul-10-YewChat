use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");
    let is_focused = use_state(|| false);

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    let onfocus = {
        let is_focused = is_focused.clone();
        Callback::from(move |_| is_focused.set(true))
    };

    let onblur = {
        let is_focused = is_focused.clone();
        Callback::from(move |_| is_focused.set(false))
    };

    // Define fixed classes without conditional logic
    let bg_classes = classes!(
        "absolute", "top-0", "left-0", "w-full", "h-full", 
        "overflow-hidden", "opacity-30"
    );

    let glow_border_classes = classes!(
        "absolute", "-inset-0.5", "bg-gradient-to-r", "from-cyan-400", 
        "via-blue-500", "to-violet-600", "rounded-2xl", 
        "blur-md", "opacity-75", "animate-pulse"
    );

    // Fixed classes for input glow - using blur-sm and opacity-30 as default
    let input_glow_classes = classes!(
        "absolute", "inset-0", "bg-gradient-to-r", "from-cyan-400", 
        "via-blue-500", "to-violet-600", "rounded-lg", "opacity-75", 
        "transition-all", "duration-300", "blur-sm", "opacity-30"
    );

    // Fixed classes for button glow - using opacity-100 and blur-sm as default
    let button_glow_classes = classes!(
        "absolute", "inset-0", "bg-gradient-to-r", "from-cyan-500", 
        "via-blue-600", "to-violet-700", "rounded-lg", 
        "transition-all", "duration-300", "opacity-100", "blur-sm"
    );

    let button_classes = classes!(
        "relative", "w-full", "flex", "justify-center", "py-3", "px-4", 
        "border", "border-transparent", "rounded-lg", "text-white", 
        "bg-gradient-to-r", "from-cyan-500", "via-blue-600", "to-violet-700", 
        "hover:from-cyan-600", "hover:via-blue-700", "hover:to-violet-800", 
        "focus:outline-none", "disabled:opacity-50", "disabled:cursor-not-allowed", 
        "transition-all", "duration-300"
    );

    let input_classes = classes!(
        "w-full", "px-4", "py-3", "bg-gray-900", "bg-opacity-50", 
        "border", "border-gray-700", "rounded-lg", "text-white", 
        "placeholder-gray-500", "focus:outline-none", "focus:ring-2", 
        "focus:ring-blue-500", "focus:border-transparent", 
        "transition-all", "duration-300"
    );

    let container_classes = classes!(
        "w-screen", "h-screen", "flex", "items-center", "justify-center", 
        "bg-black", "overflow-hidden", "relative"
    );

    let indicator_classes = classes!(
        "inline-flex", "items-center", "px-2.5", "py-0.5", "rounded-full", 
        "text-xs", "font-mono", "bg-blue-900", "bg-opacity-50", "text-blue-200"
    );

    let link_classes = classes!("block");

    html! {
        <div class={container_classes}>
            <div class={bg_classes}>
                <div class="absolute top-1/4 left-1/4 w-2/3 h-2/3 rounded-full bg-blue-700 blur-3xl animate-pulse"></div>
                <div class="absolute bottom-1/4 right-1/4 w-1/2 h-1/2 rounded-full bg-violet-700 blur-3xl animate-pulse"></div>
                <div class="absolute top-1/3 right-1/3 w-1/3 h-1/3 rounded-full bg-cyan-500 blur-3xl animate-pulse"></div>
            </div>
            
            <div class="absolute inset-0 bg-black opacity-50" style="background-image: linear-gradient(0deg, transparent 24%, rgba(70, 70, 70, .2) 25%, rgba(70, 70, 70, .2) 26%, transparent 27%, transparent 74%, rgba(70, 70, 70, .2) 75%, rgba(70, 70, 70, .2) 76%, transparent 77%, transparent), linear-gradient(90deg, transparent 24%, rgba(70, 70, 70, .2) 25%, rgba(70, 70, 70, .2) 26%, transparent 27%, transparent 74%, rgba(70, 70, 70, .2) 75%, rgba(70, 70, 70, .2) 76%, transparent 77%, transparent); background-size: 50px 50px;"></div>
            
            <div class="z-10 w-full max-w-md relative">
                <div class={glow_border_classes}></div>
                
                <div class="relative p-7 bg-black bg-opacity-80 backdrop-blur-xl rounded-xl border border-gray-800">
                    <div class="text-center">
                        <h1 class="text-3xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 via-blue-500 to-violet-600">{"NEXUS CONNECT"}</h1>
                        <div class="w-1/4 h-1 mx-auto my-3 bg-gradient-to-r from-cyan-400 via-blue-500 to-violet-600 rounded-full"></div>
                        <p class="text-gray-400 text-sm">{"SYSTEM ACCESS PROTOCOL"}</p>
                    </div>
                    
                    <div class="mt-8 space-y-5">
                        <div class="relative">
                            <div class={input_glow_classes}></div>
                            <div class="relative">
                                <label for="username" class="block text-xs font-mono tracking-widest text-gray-400 uppercase mb-1 ml-1">
                                    {"User Identifier"}
                                </label>
                                <input
                                    id="username"
                                    type="text"
                                    required=true
                                    {oninput}
                                    {onfocus}
                                    {onblur}
                                    class={input_classes}
                                    placeholder="Enter your username"
                                />
                            </div>
                        </div>
                        
                        <div class="relative">
                            <Link<Route> to={Route::Chat} classes={link_classes}>
                                <div class={button_glow_classes}></div>
                                <button
                                    {onclick}
                                    disabled={username.len() < 1}
                                    class={button_classes}
                                >
                                    <span class="absolute inset-0 flex items-center justify-center">
                                        {"INITIALIZE SESSION"}
                                    </span>
                                </button>
                            </Link<Route>>
                        </div>
                    </div>
                    
                    <div class="mt-6 text-center">
                        <div class={indicator_classes}>
                            <span class="w-2 h-2 mr-2 bg-blue-400 rounded-full animate-pulse"></span>
                            {"SECURE CONNECTION ESTABLISHED"}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
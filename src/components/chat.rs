use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
    OnKeyPress(KeyboardEvent),
    UpdateInput(String),
}

#[derive(Clone, Debug, Deserialize)]
struct MessageData {
    from: String,
    message: String,
    timestamp: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
    Error,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone, Debug)]
struct UserProfile {
    name: String,
    avatar: String,
    is_online: bool,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    input_value: String,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    is_connected: bool,
    error_message: Option<String>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Get user from context
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        
        // Create WebSocket service
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        // Create registration message
        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        // Try to send registration message
        let is_connected = match serde_json::to_string(&message) {
            Ok(json) => wss.tx.clone().try_send(json).is_ok(),
            Err(e) => {
                log::error!("Failed to serialize message: {:?}", e);
                false
            }
        };

        log::debug!("User registration status: {}", is_connected);

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            input_value: String::new(),
            wss,
            is_connected,
            error_message: None,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                match serde_json::from_str::<WebSocketMessage>(&s) {
                    Ok(msg) => {
                        self.error_message = None;
                        match msg.message_type {
                            MsgTypes::Users => {
                                let users_from_message = msg.data_array.unwrap_or_default();
                                self.users = users_from_message
                                    .iter()
                                    .map(|u| UserProfile {
                                        name: u.clone(),
                                        avatar: format!(
                                            "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                            u
                                        ),
                                        is_online: true,
                                    })
                                    .collect();
                                self.is_connected = true;
                                return true;
                            }
                            MsgTypes::Message => {
                                if let Some(data) = msg.data {
                                    match serde_json::from_str::<MessageData>(&data) {
                                        Ok(message_data) => {
                                            self.messages.push(message_data);
                                            return true;
                                        }
                                        Err(e) => {
                                            log::error!("Failed to parse message data: {:?}", e);
                                            self.error_message = Some(format!("Failed to parse message data: {}", e));
                                            return true;
                                        }
                                    }
                                }
                                return false;
                            }
                            MsgTypes::Error => {
                                self.error_message = msg.data;
                                self.is_connected = false;
                                return true;
                            }
                            _ => {
                                return false;
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to parse websocket message: {:?}", e);
                        self.error_message = Some(format!("Failed to parse server message: {}", e));
                        return true;
                    }
                }
            }
            Msg::UpdateInput(value) => {
                self.input_value = value;
                true // Mark component for re-render
            }
            Msg::OnKeyPress(e) => {
                if e.key() == "Enter" {
                    self.send_message();
                    return true;
                }
                false
            }
            Msg::SubmitMessage => {
                self.send_message();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let on_input = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateInput(input.value())
        });
        let on_keypress = ctx.link().callback(Msg::OnKeyPress);

        html! {
            <div class="flex w-screen h-screen bg-gray-900 text-gray-100">
                // Futuristic sidebar
                <div class="flex-none w-72 h-full bg-gradient-to-b from-slate-800 to-slate-900 border-r border-cyan-500/30 shadow-2xl backdrop-blur-sm">
                    <div class="sticky top-0 bg-slate-800/90 border-b border-cyan-400/20 p-6 backdrop-blur-md">
                        <div class="flex items-center space-x-3">
                            <div class="w-3 h-3 bg-cyan-400 rounded-full animate-pulse shadow-lg shadow-cyan-400/50"></div>
                            <h2 class="text-xl font-bold bg-gradient-to-r from-cyan-400 to-blue-500 bg-clip-text text-transparent">
                                {"NEURAL LINK"}
                            </h2>
                        </div>
                        <div class="mt-2 flex items-center space-x-2">
                            <span class="text-sm text-cyan-300/80">{format!("{} nodes active", self.users.len())}</span>
                            <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
                        </div>
                    </div>
                    
                    <div class="overflow-y-auto h-full pb-24 px-4 py-4 space-y-3">
                        {
                            self.users.iter().map(|u| {
                                html!{
                                    <div class="group p-4 bg-gradient-to-r from-slate-700/50 to-slate-800/50 border border-cyan-500/20 rounded-xl hover:border-cyan-400/50 hover:shadow-lg hover:shadow-cyan-400/10 transition-all duration-300 backdrop-blur-sm hover:scale-[1.02]">
                                        <div class="flex items-center space-x-3">
                                            <div class="relative">
                                                <div class="w-12 h-12 rounded-full bg-gradient-to-r from-cyan-400 to-blue-500 p-0.5">
                                                    <img class="w-full h-full rounded-full border-2 border-slate-800" 
                                                         src={u.avatar.clone()} 
                                                         alt={format!("{}'s neural avatar", u.name)}/>
                                                </div>
                                                <div class="absolute -bottom-1 -right-1 w-4 h-4 bg-green-400 border-2 border-slate-800 rounded-full shadow-lg shadow-green-400/50 animate-pulse"></div>
                                            </div>
                                            <div class="flex-1 min-w-0">
                                                <div class="text-sm font-semibold text-cyan-100 truncate group-hover:text-cyan-300 transition-colors">
                                                    {&u.name}
                                                </div>
                                                <div class="text-xs text-green-400 font-medium tracking-wide">
                                                    {"◉ ONLINE"}
                                                </div>
                                            </div>
                                            <div class="w-2 h-8 bg-gradient-to-t from-cyan-500/20 to-cyan-400/40 rounded-full opacity-60 group-hover:opacity-100 transition-opacity"></div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>

                // Futuristic main chat area
                <div class="flex-1 h-full flex flex-col bg-gradient-to-br from-slate-900 via-gray-900 to-slate-800">
                    // Futuristic header
                    <div class="flex-none h-20 bg-gradient-to-r from-slate-800/80 to-slate-700/80 border-b border-cyan-500/30 backdrop-blur-md shadow-lg">
                        <div class="flex items-center justify-between h-full px-8">
                            <div class="flex items-center space-x-4">
                                <div class="relative">
                                    <div class="w-8 h-8 bg-gradient-to-r from-cyan-400 to-blue-500 rounded-lg flex items-center justify-center shadow-lg shadow-cyan-400/30">
                                        <span class="text-slate-900 text-lg font-bold">{"⚡"}</span>
                                    </div>
                                    <div class="absolute -top-1 -right-1 w-3 h-3 bg-green-400 rounded-full animate-ping"></div>
                                </div>
                                <div>
                                    <h1 class="text-xl font-bold bg-gradient-to-r from-cyan-400 via-blue-400 to-purple-500 bg-clip-text text-transparent">
                                        {"QUANTUM CHAT NEXUS"}
                                    </h1>
                                    <div class="flex items-center space-x-2 mt-1">
                                        <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
                                        <span class="text-xs text-green-400 font-semibold tracking-wider">{"NEURAL LINK ACTIVE"}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="flex items-center space-x-3">
                                <div class="px-3 py-1 bg-cyan-500/20 border border-cyan-400/30 rounded-full">
                                    <span class="text-xs text-cyan-300 font-mono">{"STATUS: SECURE"}</span>
                                </div>
                                {
                                    if let Some(error) = &self.error_message {
                                        html! {
                                            <div class="px-3 py-1 bg-red-500/20 border border-red-400/30 rounded-full">
                                                <span class="text-xs text-red-300 font-mono">{format!("ERROR: {}", error)}</span>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
                        </div>
                    </div>

                    // Futuristic messages area
                    <div class="flex-1 overflow-y-auto p-6 space-y-6 bg-gradient-to-b from-transparent to-slate-900/50">
                        {
                            if self.messages.is_empty() {
                                html! {
                                    <div class="flex items-center justify-center h-full">
                                        <div class="text-slate-400 text-center max-w-md">
                                            <div class="w-16 h-16 mx-auto mb-4 bg-gradient-to-r from-cyan-500/20 to-blue-500/20 rounded-full flex items-center justify-center">
                                                <span class="text-2xl">{"⚡"}</span>
                                            </div>
                                            <h3 class="text-lg font-semibold text-cyan-300 mb-2">{"No Neural Transmissions Yet"}</h3>
                                            <p class="text-sm text-slate-400">{"Begin your quantum conversation by sending a neural transmission below."}</p>
                                        </div>
                                    </div>
                                }
                            } else {
                                self.messages.iter().map(|m| {
                                    let user = self.users.iter()
                                        .find(|u| u.name == m.from)
                                        .cloned()
                                        .unwrap_or_else(|| UserProfile {
                                            name: m.from.clone(),
                                            avatar: format!(
                                                "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                                m.from
                                            ),
                                            is_online: false,
                                        });

                                    html!{
                                        <div class="flex items-start space-x-4 max-w-4xl group">
                                            <div class="flex-shrink-0">
                                                <div class="w-10 h-10 rounded-full bg-gradient-to-r from-cyan-400 to-blue-500 p-0.5 shadow-lg shadow-cyan-400/30">
                                                    <img class="w-full h-full rounded-full border border-slate-700" 
                                                        src={user.avatar} 
                                                        alt={format!("{}'s neural avatar", user.name)}/>
                                                </div>
                                            </div>
                                            <div class="flex-1 bg-gradient-to-br from-slate-800/60 to-slate-700/40 border border-cyan-500/20 rounded-2xl p-5 backdrop-blur-sm shadow-xl group-hover:border-cyan-400/40 group-hover:shadow-cyan-400/10 transition-all duration-300">
                                                <div class="flex items-center space-x-3 mb-3">
                                                    <span class="text-sm font-bold text-cyan-300">{&m.from}</span>
                                                    <div class="w-1 h-1 bg-cyan-400 rounded-full"></div>
                                                    <span class="text-xs text-slate-400 font-mono">{"NEURAL_TRANSMISSION"}</span>
                                                </div>
                                                <div class="text-gray-100 leading-relaxed">
                                                    {
                                                        // Check if the message is a URL to an image
                                                        if m.message.starts_with("http") && (
                                                            m.message.ends_with(".jpg") || 
                                                            m.message.ends_with(".jpeg") || 
                                                            m.message.ends_with(".png") || 
                                                            m.message.ends_with(".gif") || 
                                                            m.message.ends_with(".webp")
                                                        ) {
                                                            html! {
                                                                <img class="mt-3 max-w-sm rounded-xl border border-cyan-500/30 shadow-lg shadow-cyan-400/20" 
                                                                    src={m.message.clone()} 
                                                                    alt="Quantum data stream"
                                                                    loading="lazy"/>
                                                            }
                                                        } else {
                                                            html! {
                                                                <p>{&m.message}</p>
                                                            }
                                                        }
                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </div>

                    // Futuristic input area
                    <div class="flex-none bg-gradient-to-r from-slate-800/90 to-slate-700/90 border-t border-cyan-500/30 p-6 backdrop-blur-md">
                        <div class="flex items-center space-x-4 max-w-6xl mx-auto">
                            <div class="flex-1 relative group">
                                <div class="absolute inset-0 bg-gradient-to-r from-cyan-500/20 to-blue-500/20 rounded-2xl blur-sm group-focus-within:blur-none transition-all duration-300"></div>
                                <input 
                                    ref={self.chat_input.clone()}
                                    type="text" 
                                    placeholder="Transmit neural message..." 
                                    class="relative w-full py-4 px-6 bg-slate-800/80 border border-cyan-500/30 rounded-2xl text-gray-100 placeholder-slate-400 focus:outline-none focus:border-cyan-400 focus:shadow-lg focus:shadow-cyan-400/20 backdrop-blur-sm transition-all duration-300 font-medium"
                                    value={self.input_value.clone()}
                                    oninput={on_input}
                                    onkeypress={on_keypress}
                                />
                                <div class="absolute right-4 top-1/2 transform -translate-y-1/2 text-slate-500">
                                    <div class="w-2 h-2 bg-cyan-400 rounded-full animate-pulse"></div>
                                </div>
                            </div>
                            <button 
                                onclick={submit}
                                class="relative p-4 bg-gradient-to-r from-cyan-500 to-blue-600 hover:from-cyan-400 hover:to-blue-500 text-white rounded-2xl shadow-lg shadow-cyan-500/30 hover:shadow-cyan-400/50 hover:scale-105 transition-all duration-300 group focus:outline-none focus:ring-2 focus:ring-cyan-400 focus:ring-offset-2 focus:ring-offset-slate-800"
                            >
                                <div class="absolute inset-0 bg-gradient-to-r from-cyan-400 to-blue-500 rounded-2xl opacity-0 group-hover:opacity-20 transition-opacity duration-300"></div>
                                <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 fill-current relative z-10">
                                    <path d="M0 0h24v24H0z" fill="none"></path>
                                    <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                                </svg>
                            </button>
                        </div>
                        <div class="flex items-center justify-center mt-4 space-x-6">
                            <div class="flex items-center space-x-2">
                                <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
                                <span class="text-xs text-green-400 font-mono">{"QUANTUM_SECURE"}</span>
                            </div>
                            <div class="w-1 h-4 bg-slate-600"></div>
                            <div class="flex items-center space-x-2">
                                <div class="w-2 h-2 bg-blue-400 rounded-full animate-pulse"></div>
                                <span class="text-xs text-blue-400 font-mono">{"NEURAL_ENCRYPTED"}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl Chat {
    fn send_message(&mut self) {
        if self.input_value.trim().is_empty() {
            return;
        }
        
        let message = WebSocketMessage {
            message_type: MsgTypes::Message,
            data: Some(self.input_value.clone()),
            data_array: None,
        };

        match serde_json::to_string(&message) {
            Ok(json) => {
                let _ = self.wss.tx.clone().try_send(json);
                self.input_value.clear();
                
                // Clear the input field
                if let Some(input) = self.chat_input.cast::<HtmlInputElement>() {
                    input.set_value("");
                }
            },
            Err(e) => {
                log::error!("Failed to serialize message: {:?}", e);
                self.error_message = Some(format!("Failed to send message: {}", e));
            }
        }
    }
}
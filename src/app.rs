mod chat;

use std::{ sync::{ Arc, Mutex }, ops::Deref, thread, borrow::Borrow };

use crate::{ error_template::{ AppError, ErrorTemplate }, app::chat::{ ChatMsg, ChatCtx } };
use leptos::{ *, html::Input };
use leptos_meta::*;
use leptos_router::*;
use serde::{ Deserialize, Serialize };

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <>

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-axum-tailwind.css"/>
        <Body class="bg-gray-900"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
                <NavBar/>
                <Content/>                
        </Router>
        </>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "userId")]
    user_id: i8,
    id: i8,
    title: String,
    completed: bool,
}

#[server(FetchTodo, "/api")]
pub async fn fetch_todo() -> Result<Todo, ServerFnError> {
    let res = reqwest
        ::get(&format!("https://jsonplaceholder.typicode.com/todos/1")).await?
        .json::<Todo>().await?;
    Ok(res)
}

#[component]
fn Content() -> impl IntoView {
    view! {
        <div class="h-screen flex">
            <Sidebar/>
            <main class="lg:flex-1 p-4">                               
                <Routes>
                    <Route path="/todo" view=HomePage/>
                    <Route path="/chat" view=ChatArea/>
                </Routes>
            </main>
        </div>
    }
}

#[component]
fn Message(chat_msg: ChatMsg) -> impl IntoView {
    let msg = format!("{}:{}", chat_msg.user_type, chat_msg.message);
    view! {
        <p class="text-white">{msg}</p>
    }
}

#[component]
fn ChatArea() -> impl IntoView {
    let mut msgs = Vec::<ChatMsg>::new();
    let chat_ctx = Arc::new(Mutex::new(ChatCtx::new()));

    let (chat_msgs, set_chat_msgs) = create_signal(msgs);
    let input_element: NodeRef<Input> = create_node_ref();

    view! {
        <div class="w-1/2 h-1/2 bg-gray-200 rounded-full border-2 border-white">
            <h1 class="text-white">Chat</h1>
            <ul>
                <For
                    each=chat_msgs
                    key=|msg| msg.get_id()
                    children = move |msg:ChatMsg|{
                        view! {
                            <li><Message chat_msg=msg/></li>
                        }
                    }                    
                />             
            </ul>
            <input type="text"
                node_ref=input_element
                class="border border-gray-300 rounded px-2 py-1 absolute bottom-4 left-4 w-1/2"/>
                <button class="bg-blue-500 text-white px-4 py-2 absolute bottom-4 right-4" 
                    on:click=move |ev| {   
                        ev.prevent_default();
                        let chat_clone = Arc::clone(&chat_ctx);   
                        let input = input_element().unwrap().value(); 
                        let mut msg_id:Option<i32> = None; 
                        msg_id = Some(chat_clone.lock().expect("cloned chat is null").add_msg("bb".to_string(),"Agent".to_string()));
                        // // logging::log!("{:?}", &input);
                        // let input = input_element().unwrap().value(); 

                        set_chat_msgs.update(move |m| {
                            m.push(ChatMsg::new(msg_id.clone().unwrap(), input.clone(),"Agent".to_string()));
                            // chat_ctx.
                        });
                    }>
            Submit
            </button>
        </div>.
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    view! {
            <aside class="lg:w-7/10 bg-gray-800 p-4 text-white">
                <ul>
                  <li class="mb-4">
                    <a href="#" class="text-gray-400 hover:text-white">Dashboard</a>
                  </li>
                  <li class="mb-4">
                    <a href="#" class="text-gray-400 hover:text-white">Messages</a>
                  </li>
                  <li class="mb-4">
                    <a href="#" class="text-gray-400 hover:text-white">Settings</a>
                  </li>
                </ul>
            </aside>
    }
}

#[component]
fn LoginForm() -> impl IntoView {
    view! {
        <div class="w-full max-w-xs">
            <form class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="username">
                        "Username"
                    </label>
                    <input class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" placeholder="Username"/>
                </div>
                //     <div class="mb-6">
                //         <label class="block text-gray-700 text-sm font-bold mb-2" for="password">
                //         "Password"
                //         </label>
                //         <input class="shadow appearance-none border border-red-500 rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline" id="password" type="password" placeholder="******************">
                //         <p class="text-red-500 text-xs italic">Please choose a password.</p>
                //     </div>
                //     <div class="flex items-center justify-between">
                //         <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="button">
                //         "Sign In"
                //         </button>
                //         <a class="inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800" href="#">
                //         "Forgot Password?"
                //         </a>
                //     </div>
            </form>       
        </div>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav class="bg-gray-900 text-white p-4">
          <div class="container mx-auto flex justify-between items-center">
            <a href="#" class="text-2xl font-bold">Logo</a>
            <div class="flex space-x-4">
              <a href="#" class="hover:text-gray-400">Home</a>
              <a href="#" class="hover:text-gray-400">About</a>
              <a href="#" class="hover:text-gray-400">Services</a>
              <a href="#" class="hover:text-gray-400">Contact</a>
            </div>
          </div>
        </nav>

    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let todo = create_resource(
        || (),
        |_| async { fetch_todo().await }
    );

    view! {
        <div>
            <Transition fallback=move || view! {<p>"Loading..."</p> }>
            {
                move || {
                    todo.get().map(|td| {
                        match td {
                            Ok(t) => { 
                                view! {
                                    <h1 class="text-white">"TEST"</h1>
                                    <p>{t.title}</p>
                                }.into_view()
                            },
                            Err(e) =>{ 
                                view! { 
                                    <p>{e.to_string()}</p> 
                                }
                            }.into_view(),
                        }
                    }).collect_view()
                }
            }
            </Transition>
        </div>
    }
}

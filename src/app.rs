use crate::error_template::{ AppError, ErrorTemplate };
use leptos::*;
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
            <main>
                <NavBar/>
                <Content/>
                    
                
                    <Routes>
                        <Route path="/todo" view=HomePage/>
                    </Routes>
            </main>
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
            <main class="w-3/4">
                <h2 class="text-2xl font-semibold mb-4 text-white">"Main Content"</h2>
            </main>

        </div>

    }
}

#[component]
fn Sidebar() -> impl IntoView {
    view! {
            <aside class="w-1/4 bg-gray-800 p-4 text-white">
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

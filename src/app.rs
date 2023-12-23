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

        // sets the document title
        <Title text="Welcome to Leptos"/>
        // <NavBar/>
        // <Sidebar/>
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
                <Routes>
                    <Route path="/todo" view=HomePage/>
                </Routes>
            </main>
        </Router>
        </>
    }
}

// {
// "userId": 1,
// "id": 1,
// "title": "delectus aut autem",
// "completed": false
// }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    userId: i8,
    id: i8,
    title: String,
    competed: bool,
}

#[server(FetchTodo, "/api")]
pub async fn fetch_todo() -> Result<Todo, ServerFnError> {
    // let res: Todo = reqwasm::http::Request
    //     ::get(&format!("https://jsonplaceholder.typicode.com/todos/1"))
    //     .send().await?
    //     .json::<Todo>().await?;

    Ok(Todo {
        userId: 1,
        title: "Title".to_string(),
        id: 0,
        competed: true,
    })
}

#[component]
fn Sidebar() -> impl IntoView {
    view! {
        <div class="h-screen flex">
            <aside class="bg-gray-800 w-64 p-4 text-white">
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
                                <h1>"TEST"</h1>
                                <p>{t.userId}</p>
                            }.into_view()
                        },
                        Err(e) =>{ 
                            view! { 
                                <p>"Error"</p> 
                            }
                        }.into_view(),
                    }
                }).collect_view()
            }
        }
        </Transition>
        </div>
    }
    // let todo = fetch_todo();
    // let x = move || {
    //     async {
    //         let tdo: Result<Todo, ServerFnError> = fetch_todo().await;
    //         let x = match tdo {
    //             Ok(_t) => {
    //                 view! {
    //                     <h1>"Test"</h1>
    //                     <p>{_t.userId}</p>
    //                 }
    //             }
    //             Err(e) => { Err(e) }
    //         };
    //     }
    // };

    // // let todo_view = todo.and_then(|x|{
    // //     view!{
    // //         <p>x.title</p>
    // //     }
    // // });

    // // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(0);
    // let on_click = move |_|
    //     set_count.update(|count| {
    //         *count += 1;
    //     });

    // view! {
    //     <h1>"Welcome to Leptos!"</h1>
    //     // <p>serde_json::to_string(&_todo)</p>
    //     <button class="bg-blue-500 dark:hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click=on_click>"Click Me: " {count}</button>
    // }
}

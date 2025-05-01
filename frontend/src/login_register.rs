use yew::prelude::*;
use gloo_net::http::Request;

#[function_component(LoginRegister)]
pub fn login_register(props: &LoginRegisterProps) -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error = use_state(|| None as Option<String>);
    let is_login = use_state(|| true);

    let on_username = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                username.set(input.value());
            }
        })
    };
    let on_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                password.set(input.value());
            }
        })
    };
    let on_toggle = {
        let is_login = is_login.clone();
        Callback::from(move |_| is_login.set(!*is_login))
    };
    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let error = error.clone();
        let is_login = is_login.clone();
        let on_success = props.on_success.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username).clone();
            let password = (*password).clone();
            let is_login = *is_login;
            let error = error.clone();
            let on_success = on_success.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let base_url = "http://127.0.0.1:8080"; // Define backend base URL
                let endpoint = if is_login { "/api/login" } else { "/api/register" };
                let url = format!("{}{}", base_url, endpoint); // Construct full URL
                let resp = Request::post(&url) // Use the full URL
                    .header("Content-Type", "application/json")
                    .body(serde_json::json!({"username": username, "password": password}).to_string())
                    .expect("Failed to build request")
                    .send()
                    .await;
                match resp {
                    Ok(r) if r.status() == 200 => {
                        on_success.emit(username);
                    }
                    Ok(r) => {
                        let msg = r.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                        error.set(Some(msg));
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                    }
                }
            });
        })
    };
    html! {
        <form onsubmit={on_submit} style="display:flex;flex-direction:column;gap:0.5rem;max-width:300px;margin:2rem auto;">
            <h2>{ if *is_login { "Login" } else { "Register" } }</h2>
            <input placeholder="Username" value={(*username).clone()} oninput={on_username} required=true />
            <input type="password" placeholder="Password" value={(*password).clone()} oninput={on_password} required=true />
            <button type="submit">{ if *is_login { "Login" } else { "Register" } }</button>
            <button type="button" onclick={on_toggle}>{ if *is_login { "Need an account? Register" } else { "Already have an account? Login" } }</button>
            { if let Some(err) = &*error { html!{ <p style="color:red;">{err}</p> } } else { html!{} } }
        </form>
    }
}

#[derive(Properties, PartialEq)]
pub struct LoginRegisterProps {
    pub on_success: Callback<String>,
}

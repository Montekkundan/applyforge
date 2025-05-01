use yew::prelude::*;
use crate::login_register::LoginRegister;
use crate::models::ApplicationStatus;
use std::collections::HashMap;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use uuid::Uuid;

const USER_KEY: &str = "applyforge_user";

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JobApplication {
    pub id: Option<i32>,
    #[serde(default = "Uuid::new_v4")]
    pub temp_id: Uuid,
    pub company: String,
    pub position: String,
    pub status: ApplicationStatus,
    pub date: String,
    pub username: String,
}

#[function_component(AppRoot)]
pub fn app_root() -> Html {
    let user = use_state(|| LocalStorage::get(USER_KEY).ok());
    let applications = use_state(|| Vec::<JobApplication>::new());
    let company = use_state(|| String::new());
    let position = use_state(|| String::new());
    let status = use_state(|| ApplicationStatus::Applied);
    let today = {
        let now = js_sys::Date::new_0();
        let year = now.get_full_year();
        let month = now.get_month() + 1;
        let day = now.get_date();
        format!("{:04}-{:02}-{:02}", year, month, day)
    };
    let date = use_state(|| today.clone());

    let on_success = {
        let user = user.clone();
        Callback::from(move |username: String| {
            LocalStorage::set(USER_KEY, &username).expect("Failed to set user in localStorage");
            user.set(Some(username));
        })
    };

    let user_dep = (*user).clone();

    {
        let applications_handle = applications.clone();
        let user_for_effect = user_dep.clone();
        use_effect_with(user_for_effect, move |user_state| {
            let applications = applications_handle.clone();
            if let Some(username) = user_state {
                let username = username.clone();
                let applications_for_async = applications.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let base_url = "http://127.0.0.1:8080";
                    let url = format!("{}/api/jobs/{}", base_url, username);
                    match Request::get(&url).send().await {
                        Ok(resp) => {
                            if resp.ok() {
                                match resp.json::<Vec<JobApplication>>().await {
                                    Ok(mut fetched_jobs) => {
                                        for job in &mut fetched_jobs {
                                            if job.temp_id == Uuid::nil() {
                                                job.temp_id = Uuid::new_v4();
                                            }
                                        }
                                        applications_for_async.set(fetched_jobs);
                                    }
                                    Err(e) => gloo::console::error!(format!("Failed to parse jobs JSON: {:?}", e))
                                }
                            } else {
                                gloo::console::error!(format!("Failed to fetch jobs: Status {}", resp.status()));
                            }
                        }
                        Err(e) => gloo::console::error!(format!("Failed to send fetch jobs request: {:?}", e))
                    }
                });
            } else {
                applications.set(Vec::new());
            }
            || ()
        });
    }

    let on_add = {
        let applications = applications.clone();
        let company = company.clone();
        let position = position.clone();
        let status = status.clone();
        let date = date.clone();
        let user = user.clone();
        let today = today.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if let Some(username) = &*user {
                let company_val = (*company).clone();
                let position_val = (*position).clone();
                let status_val = (*status).clone();
                let date_val = (*date).clone();

                let temp_job = JobApplication {
                    id: None,
                    temp_id: Uuid::new_v4(),
                    company: company_val.clone(),
                    position: position_val.clone(),
                    status: status_val.clone(),
                    date: date_val.clone(),
                    username: username.clone(),
                };

                let mut current_apps = (*applications).clone();
                current_apps.push(temp_job.clone());
                applications.set(current_apps);

                company.set(String::new());
                position.set(String::new());
                date.set(today.clone());

                let applications_clone = applications.clone();
                let username_clone = username.clone();
                let temp_job_clone = temp_job.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let base_url = "http://127.0.0.1:8080";
                    let url = format!("{}/api/jobs", base_url);

                    let backend_job_data = serde_json::json!({
                        "company": temp_job_clone.company,
                        "position": temp_job_clone.position,
                        "status": temp_job_clone.status,
                        "date": temp_job_clone.date,
                        "username": username_clone
                    });

                    match Request::post(&url)
                        .header("Content-Type", "application/json")
                        .body(backend_job_data.to_string())
                    {
                        Ok(request) => {
                            match request.send().await {
                                Ok(resp) => {
                                    if resp.ok() {
                                        match resp.json::<JobApplication>().await {
                                            Ok(mut confirmed_job) => {
                                                let mut current_apps = (*applications_clone).clone();
                                                if let Some(index) = current_apps.iter().position(|j| j.temp_id == temp_job_clone.temp_id) {
                                                    if confirmed_job.temp_id == Uuid::nil() {
                                                        confirmed_job.temp_id = temp_job_clone.temp_id;
                                                    }
                                                    current_apps[index] = confirmed_job;
                                                    applications_clone.set(current_apps);
                                                } else {
                                                    gloo::console::warn!("Optimistically added job not found after confirmation?");
                                                }
                                            }
                                            Err(e) => {
                                                gloo::dialogs::alert(&format!("Failed to parse add job response: {:?}", e));
                                                let mut current_apps = (*applications_clone).clone();
                                                current_apps.retain(|j| j.temp_id != temp_job_clone.temp_id);
                                                applications_clone.set(current_apps);
                                            }
                                        }
                                    } else {
                                        let error_body = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                                        gloo::dialogs::alert(&format!("Failed to add job: {} - {}", resp.status(), error_body));
                                        let mut current_apps = (*applications_clone).clone();
                                        current_apps.retain(|j| j.temp_id != temp_job_clone.temp_id);
                                        applications_clone.set(current_apps);
                                    }
                                }
                                Err(e) => {
                                    gloo::dialogs::alert(&format!("Failed to send add job request: {:?}", e));
                                    let mut current_apps = (*applications_clone).clone();
                                    current_apps.retain(|j| j.temp_id != temp_job_clone.temp_id);
                                    applications_clone.set(current_apps);
                                }
                            }
                        }
                        Err(e) => {
                            gloo::dialogs::alert(&format!("Failed to build add job request: {}", e));
                            let mut current_apps = (*applications_clone).clone();
                            current_apps.retain(|j| j.temp_id != temp_job_clone.temp_id);
                            applications_clone.set(current_apps);
                        }
                    }
                });
            } else {
                gloo::dialogs::alert("You must be logged in to add a job.");
            }
        })
    };

    let on_company = {
        let company = company.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                company.set(input.value());
            }
        })
    };
    let on_position = {
        let position = position.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                position.set(input.value());
            }
        })
    };
    let on_date = {
        let date = date.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                date.set(input.value());
            }
        })
    };
    let on_status = {
        let status = status.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                let value = select.value();
                let s = match value.as_str() {
                    "Applied" => ApplicationStatus::Applied,
                    "Interview" => ApplicationStatus::Interview,
                    "Offer" => ApplicationStatus::Offer,
                    "Rejected" => ApplicationStatus::Rejected,
                    "Accepted" => ApplicationStatus::Accepted,
                    _ => ApplicationStatus::Applied,
                };
                status.set(s);
            }
        })
    };

    if user.is_none() {
        html! { <LoginRegister on_success={on_success} /> }
    } else {
        let mut status_counts: HashMap<&str, usize> = HashMap::new();
        for app in applications.iter() {
            let key = match app.status {
                ApplicationStatus::Applied => "Applied",
                ApplicationStatus::Interview => "Interview",
                ApplicationStatus::Offer => "Offer",
                ApplicationStatus::Rejected => "Rejected",
                ApplicationStatus::Accepted => "Accepted",
            };
            *status_counts.entry(key).or_insert(0) += 1;
        }

        html! {
            <main style="max-width:600px;margin:2rem auto;padding:2rem;background:oklch(0.205 0 0 / 0.9);border-radius:1rem;box-shadow:0 2px 16px #0008;color:oklch(0.9 0 0);">
                <h1 style="text-align:center;color:oklch(0.985 0 0);">{ "Job Application Tracker" }</h1>
                <p style="text-align:right;">{format!("Logged in as: {}", user.as_ref().unwrap())}</p>

                <form onsubmit={on_add} style="display:flex;flex-direction:column;gap:0.5rem;margin-bottom:2rem;">
                    <input placeholder="Company" value={(*company).clone()} oninput={on_company} required=true style="padding:0.5rem;border-radius:0.3rem;border:1px solid #555;background:#333;color:#eee;"/>
                    <input placeholder="Position" value={(*position).clone()} oninput={on_position} required=true style="padding:0.5rem;border-radius:0.3rem;border:1px solid #555;background:#333;color:#eee;"/>
                    <select onchange={on_status} value={format!("{:?}", *status)} style="padding:0.5rem;border-radius:0.3rem;border:1px solid #555;background:#333;color:#eee;">
                        <option value="Applied" selected={matches!(*status, ApplicationStatus::Applied)}>{"Applied"}</option>
                        <option value="Interview" selected={matches!(*status, ApplicationStatus::Interview)}>{"Interview"}</option>
                        <option value="Offer" selected={matches!(*status, ApplicationStatus::Offer)}>{"Offer"}</option>
                        <option value="Rejected" selected={matches!(*status, ApplicationStatus::Rejected)}>{"Rejected"}</option>
                        <option value="Accepted" selected={matches!(*status, ApplicationStatus::Accepted)}>{"Accepted"}</option>
                    </select>
                    <input type="date" value={(*date).clone()} oninput={on_date} required=true style="padding:0.5rem;border-radius:0.3rem;border:1px solid #555;background:#333;color:#eee;"/>
                    <button type="submit" style="background:#007bff;color:#fff;padding:0.5rem 1rem;border:none;border-radius:0.5rem;cursor:pointer;">{"Add Job"}</button>
                </form>

                <section style="margin-bottom:2rem;">
                    <h2 style="border-bottom: 1px solid #444; padding-bottom: 0.5rem;">{ "Analytics" }</h2>
                    <ul style="display:flex;flex-wrap:wrap;gap:1rem;list-style:none;padding:0;">
                        { for status_counts.iter().map(|(status, count)| html!{
                            <li style="background:oklch(0.3 0 0);padding:0.5rem 1rem;border-radius:0.5rem;">{ format!("{}: {}", status, count) }</li>
                        }) }
                    </ul>
                </section>

                <section style="margin-bottom:2rem;">
                    <h2 style="border-bottom: 1px solid #444; padding-bottom: 0.5rem;">{ "Applications" }</h2>
                    <table style="width:100%;border-collapse:collapse;color:oklch(0.85 0 0);">
                        <thead>
                            <tr style="background:oklch(0.3 0 0);">
                                <th style="padding:0.5rem;border-bottom:1px solid #555;text-align:left;">{ "Company" }</th>
                                <th style="padding:0.5rem;border-bottom:1px solid #555;text-align:left;">{ "Position" }</th>
                                <th style="padding:0.5rem;border-bottom:1px solid #555;text-align:left;">{ "Status" }</th>
                                <th style="padding:0.5rem;border-bottom:1px solid #555;text-align:left;">{ "Date" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for applications.iter().map(|app| html! {
                                <tr class="job-row" key={app.temp_id.to_string()} style="border-bottom:1px solid #444;">
                                    <td style="padding:0.5rem;">{ &app.company }</td>
                                    <td style="padding:0.5rem;">{ &app.position }</td>
                                    <td style="padding:0.5rem;">{ format!("{:?}", app.status) }</td>
                                    <td style="padding:0.5rem;">{ &app.date }</td>
                                </tr>
                            }) }
                        </tbody>
                    </table>
                     { if applications.is_empty() {
                        html!{ <p style="text-align:center;margin-top:1rem;color:#888;">{"No applications added yet."}</p> }
                       } else { html!{} }
                     }
                </section>

                <section style="display:flex;gap:1rem;justify-content:center;margin-top:2rem;">
                    <button style="background:#28a745;color:#fff;padding:0.5rem 1rem;border:none;border-radius:0.5rem;cursor:pointer;">{ "Export to CSV" }</button>
                     <button onclick={Callback::from(move |_| {
                         LocalStorage::delete(USER_KEY);
                         user.set(None);
                     })} style="background:#6c757d;color:#fff;padding:0.5rem 1rem;border:none;border-radius:0.5rem;cursor:pointer;">{ "Logout" }</button>
                </section>
            </main>
        }
    }
}
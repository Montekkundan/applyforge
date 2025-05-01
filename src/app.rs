use yew::prelude::*;
use crate::models::{JobApplication, ApplicationStatus};
use std::collections::HashMap;

#[yew::function_component(App)]
pub fn app() -> Html {
    // State for job applications and form fields
    let applications = use_state(|| Vec::<JobApplication>::new());
    let company = use_state(|| String::new());
    let position = use_state(|| String::new());
    let status = use_state(|| ApplicationStatus::Applied);
    // Set default date to today (YYYY-MM-DD)
    let today = {
        let now = js_sys::Date::new_0();
        let year = now.get_full_year();
        let month = now.get_month() + 1; // JS months are 0-based
        let day = now.get_date();
        format!("{:04}-{:02}-{:02}", year, month, day)
    };
    let date = use_state(|| today);

    let on_add = {
        let applications = applications.clone();
        let company = company.clone();
        let position = position.clone();
        let status = status.clone();
        let date = date.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let mut apps = (*applications).clone();
            apps.push(JobApplication {
                id: apps.len() + 1,
                company: (*company).clone(),
                position: (*position).clone(),
                status: (*status).clone(),
                date: (*date).clone(),
            });
            applications.set(apps);
            company.set(String::new());
            position.set(String::new());
            date.set(String::new());
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

    // Analytics: count by status
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
        <main style="max-width:600px;margin:2rem auto;padding:2rem;background:oklch(0.205 0 0);border-radius:1rem;box-shadow:0 2px 16px #0008;">
            <h1 style="text-align:center;color:oklch(0.985 0 0);">{ "Job Application Tracker" }</h1>
            <form onsubmit={on_add} style="display:flex;flex-direction:column;gap:0.5rem;margin-bottom:2rem;">
                <input placeholder="Company" value={(*company).clone()} oninput={on_company} required=true />
                <input placeholder="Position" value={(*position).clone()} oninput={on_position} required=true />
                <select onchange={on_status}>
                    <option selected={matches!(*status, ApplicationStatus::Applied)}>{"Applied"}</option>
                    <option selected={matches!(*status, ApplicationStatus::Interview)}>{"Interview"}</option>
                    <option selected={matches!(*status, ApplicationStatus::Offer)}>{"Offer"}</option>
                    <option selected={matches!(*status, ApplicationStatus::Rejected)}>{"Rejected"}</option>
                    <option selected={matches!(*status, ApplicationStatus::Accepted)}>{"Accepted"}</option>
                </select>
                <input type="date" value={(*date).clone()} oninput={on_date} required=true />
                <button type="submit" style="background:#007bff;color:#fff;padding:0.5rem 1rem;border:none;border-radius:0.5rem;">{"Add Job"}</button>
            </form>
            <section style="margin-bottom:2rem;">
                <h2>{ "Analytics" }</h2>
                <ul style="display:flex;gap:1rem;list-style:none;padding:0;">
                    { for status_counts.iter().map(|(status, count)| html!{
                        <li style="background:#f0f0f0;padding:0.5rem 1rem;border-radius:0.5rem;">{ format!("{}: {}", status, count) }</li>
                    }) }
                </ul>
            </section>
            <section style="margin-bottom:2rem;">
                <h2>{ "Applications" }</h2>
                <table style="width:100%;border-collapse:collapse;">
                    <thead>
                        <tr style="background:#f8f8f8;">
                            <th style="padding:0.5rem;border-bottom:1px solid #ddd;">{ "Company" }</th>
                            <th style="padding:0.5rem;border-bottom:1px solid #ddd;">{ "Position" }</th>
                            <th style="padding:0.5rem;border-bottom:1px solid #ddd;">{ "Status" }</th>
                            <th style="padding:0.5rem;border-bottom:1px solid #ddd;">{ "Date" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for applications.iter().map(|app| html! {
                            <tr>
                                <td style="padding:0.5rem;border-bottom:1px solid #eee;">{ &app.company }</td>
                                <td style="padding:0.5rem;border-bottom:1px solid #eee;">{ &app.position }</td>
                                <td style="padding:0.5rem;border-bottom:1px solid #eee;">{ match app.status {
                                    ApplicationStatus::Applied => "Applied",
                                    ApplicationStatus::Interview => "Interview",
                                    ApplicationStatus::Offer => "Offer",
                                    ApplicationStatus::Rejected => "Rejected",
                                    ApplicationStatus::Accepted => "Accepted",
                                } }</td>
                                <td style="padding:0.5rem;border-bottom:1px solid #eee;">{ &app.date }</td>
                            </tr>
                        }) }
                    </tbody>
                </table>
            </section>
            <section style="display:flex;gap:1rem;justify-content:center;">
                <button style="background:#28a745;color:#fff;padding:0.5rem 1rem;border:none;border-radius:0.5rem;">{ "Export to CSV" }</button>
                <button style="background:#db4437;color:#fff;padding:0.5rem 1rem;border:none;border-radius:0.5rem;">{ "Login with OAuth" }</button>
            </section>
        </main>
    }
}

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ApplicationStatus {
    Applied,
    Interview,
    Offer,
    Rejected,
    Accepted,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct JobApplication {
    pub id: usize,
    pub company: String,
    pub position: String,
    pub status: ApplicationStatus,
    pub date: String,
    pub username: String,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicationStatus {
    Applied,
    Interview,
    Offer,
    Rejected,
    Accepted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobApplication {
    pub id: usize,
    pub company: String,
    pub position: String,
    pub status: ApplicationStatus,
    pub date: String,
    pub username: String, 
}

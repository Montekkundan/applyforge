#[derive(Clone, PartialEq, Debug)]
pub enum ApplicationStatus {
    Applied,
    Interview,
    Offer,
    Rejected,
    Accepted,
}

#[derive(Clone, PartialEq, Debug)]
pub struct JobApplication {
    pub id: usize,
    pub company: String,
    pub position: String,
    pub status: ApplicationStatus,
    pub date: String, 
}

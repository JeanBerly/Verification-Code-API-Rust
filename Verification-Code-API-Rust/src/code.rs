use std::time::SystemTime;
#[derive(Debug)]
pub struct VerificationCode{
    pub code: String,
    pub name: String,
    pub email: String,
    pub emission_time: SystemTime,
}
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::models::ApiResponse;

impl<T: Serialize> ApiResponse<T> {

    fn latency(t_in: DateTime<Utc>, t_out: DateTime<Utc>) -> String {
        format!("{}ms", (t_out - t_in).num_milliseconds())
    }
    pub fn success(t_in: DateTime<Utc>, status: u16, data: T) -> Self {
        
        let t_out = Utc::now();
        let latency = Self::latency(t_in, t_out);

        ApiResponse {
            latency,
            tin: t_in,
            tout: t_out,
            data: Some(data),
            error: None,
            status,
            success: true,
        }
    }

    pub fn error(t_in: DateTime<Utc>, status: u16, err: String) -> Self {

        let t_out = Utc::now();
        let latency = Self::latency(t_in, t_out);

        ApiResponse {
            latency,
            tin: t_in,
            tout: t_out,
            data: None,
            error: Some(err),
            status,
            success: false,
        }
    }
}
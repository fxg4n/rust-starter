#[macro_export]
macro_rules! log_error {
    ($err:expr) => {{
        tracing::error!(
            { ERROR_TYPE_FIELD } = std::any::type_name_of_val(&$err),
            { ERROR_DISPLAY_FIELD } = %$err,
            { ERROR_DEBUG_FIELD } = ?$err,
            { TIMESTAMP_FIELD } = time::OffsetDateTime::now_utc().to_string(),
            "An error occurred"
        );
    }};
    ($err:expr, $message:expr) => {{
        tracing::error!(
            error.type_name = std::any::type_name_of_val(&$err),
            error.display = %$err,
            error.debug = ?$err,
            error.timestamp = time::OffsetDateTime::now_utc().to_string(),
            $message
        );
    }};
}

#[macro_export]
macro_rules! log_request {
    ($req:expr) => {{
        let request_id = $crate::core::logging::request_id::RequestId::new();
        tracing::info!(
            { REQUEST_ID_FIELD } = %request_id.as_str(),
            request.method = %$req.method(),
            request.uri = %$req.uri(),
            request.version = ?$req.version(),
            request.headers = ?$req.headers(),
            { TIMESTAMP_FIELD } = time::OffsetDateTime::now_utc().to_string(),
            "Incoming request"
        );
        request_id
    }};
}

#[macro_export]
macro_rules! log_response {
    ($response:expr, $duration:expr, $request_id:expr) => {{
        tracing::info!(
            response.status = %$response.status(),
            response.duration_ms = $duration.as_millis() as u64,
            response.request_id = %$request_id.as_str(),
            response.timestamp = time::OffsetDateTime::now_utc().to_string(),
            "Outgoing response"
        );
    }};
}

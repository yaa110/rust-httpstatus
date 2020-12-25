use httpstatus::{StatusClass, StatusCode};

#[test]
fn cast_to_test() {
    assert_eq!(StatusCode::Continue, 100.into());
    assert_eq!(StatusCode::Ok, 200.into());
    assert_eq!(StatusCode::MultipleChoices, 300.into());
    assert_eq!(StatusCode::BadRequest, 400.into());
    assert_eq!(StatusCode::InternalServerError, 500.into());
    assert_eq!(StatusCode::Unknown(1000), 1000.into());
}

#[test]
fn cast_from_test() {
    assert_eq!(StatusCode::Continue.as_u16(), 100);
    assert_eq!(StatusCode::Ok.as_u16(), 200);
    assert_eq!(StatusCode::MultipleChoices.as_u16(), 300);
    assert_eq!(StatusCode::BadRequest.as_u16(), 400);
    assert_eq!(StatusCode::InternalServerError.as_u16(), 500);
    assert_eq!(StatusCode::Unknown(1000).as_u16(), 1000);
}

#[test]
fn class_test() {
    assert_eq!(StatusCode::Continue.class(), StatusClass::Informational);
    assert_eq!(StatusCode::Ok.class(), StatusClass::Success);
    assert_eq!(
        StatusCode::MultipleChoices.class(),
        StatusClass::Redirection
    );
    assert_eq!(StatusCode::BadRequest.class(), StatusClass::ClientError);
    assert_eq!(
        StatusCode::InternalServerError.class(),
        StatusClass::ServerError
    );
    assert_eq!(StatusCode::Unknown(1000).class(), StatusClass::Unknown);
}

#[test]
fn display_test() {
    assert_eq!(StatusCode::Continue.to_string(), "100 Continue");
    assert_eq!(StatusCode::Ok.to_string(), "200 OK");
    assert_eq!(
        StatusCode::MultipleChoices.to_string(),
        "300 Multiple Choices"
    );
    assert_eq!(StatusCode::BadRequest.to_string(), "400 Bad Request");
    assert_eq!(
        StatusCode::InternalServerError.to_string(),
        "500 Internal Server Error"
    );
    assert_eq!(StatusCode::Unknown(1000).to_string(), "1000");
}

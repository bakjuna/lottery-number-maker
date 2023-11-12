#[cfg(test)]
mod tests {

    use crate::{
        lottery::{
            handler::handler_lottery,
            service::{DynLotteryService, MockLotteryServiceTrait},
        },
        EnvVars, MockAppStateTrait,
    };
    use std::sync::Arc;

    use axum::{
        http::{self, Request, StatusCode},
        routing::get,
        Router,
    };
    use hyper::body::to_bytes;
    use rstest::rstest;
    use serde_json::{Map, Value};
    use tower::ServiceExt;

    #[rstest]
    #[tokio::test]
    async fn test_real_random_number() {
        let mut mock_state = MockAppStateTrait::new();
        mock_state.expect_get_env().returning(EnvVars::new);
        mock_state.expect_get_lottery_service().returning(|| {
            let mut mock_lottery_service = MockLotteryServiceTrait::new();
            mock_lottery_service
                .expect_is_distributed()
                .returning(|_x| false);
            mock_lottery_service
                .expect_generate_number()
                .returning(|| Some(vec![1, 2, 3, 4, 5, 6]));
            Arc::new(mock_lottery_service) as DynLotteryService
        });
        let state = Arc::new(mock_state);
        let router = Router::new();
        let app = router.route("/", get(handler_lottery)).with_state(state);
        let req = Request::builder()
            .method(http::Method::GET)
            .uri("/")
            .body("".to_string())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let byte = to_bytes(res.into_body()).await.unwrap();
        let body: Map<String, Value> = serde_json::from_slice(&byte.to_ascii_lowercase()).unwrap();
        let numbers: Vec<i64> = body
            .get("numbers")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|el: &Value| el.as_i64().unwrap())
            .collect();
        assert_eq!(numbers, vec![1, 2, 3, 4, 5, 6])
    }
}

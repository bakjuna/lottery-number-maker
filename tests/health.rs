use anyhow::Result;
use httpc_test::Response;

#[tokio::test]
async fn health() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;
	hc.do_get("/healthz").await?.print().await?;
	Ok(())
}

#[tokio::test]
async fn lottery() -> Result<()> {
	// given
	let hc = httpc_test::new_client("http://localhost:8080")?;

	// when
	let response: Response = hc.do_get("/lottery").await?;

	// then
	assert_eq!(response.status(), 200);
	// let c = response.json_value::<String>("").unwrap();
	let value = response.json_body().unwrap();
	let a = value.get("title").ok_or("err").unwrap();
	assert_eq!(a, "test");
	Ok(())
}
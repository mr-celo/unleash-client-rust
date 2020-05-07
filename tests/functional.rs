// Copyright 2020 Cognite AS

//! Functional test against an unleashed API server running locally.
//! Set environment variables as per config.rs to exercise this.
//!
//! Currently expects a feature called default with one strategy default
//! Additional features are ignored.

#[cfg(all(feature = "functional"))]
mod tests {

    use async_std::task;
    use std::time::Duration;

    use futures_timer::Delay;

    use unleash_api_client::client;
    use unleash_api_client::config::EnvironmentConfig;

    #[test]
    fn test_register() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let _ = simple_logger::init();
        task::block_on(async {
            let config = EnvironmentConfig::from_env()?;
            let mut builder = client::ClientBuilder::default();
            builder.interval(500);
            let client = builder.into_client::<http_client::native::NativeClient>(
                &config.api_url,
                &config.app_name,
                &config.instance_id,
                config.secret,
            )?;
            client.register().await?;
            futures::future::join(client.poll(), async {
                Delay::new(Duration::from_millis(1500)).await;
                assert_eq!(true, client.is_enabled("default", None, false));
                client.stop_poll().await;
            })
            .await;
            println!("got metrics");
            Ok(())
        })
    }
}

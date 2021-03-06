#[cfg(test)]
mod actix_tests {

    use actix_web::test;
    use actix_web::{web, App};
    use device::{rdevice::RDevice, rwdevice::RWDevice, Readable};
    use serde_json::json;

    use crate::handler::*;

    #[actix_rt::test]
    async fn get_forecast() {
        let mut test = test::init_service(
            App::new().service(
                web::scope("/rdevices")
                    .route("/weather/forecast/{day}", web::get().to(get_fake_forecast)),
            ),
        )
        .await;

        let request = test::TestRequest::get()
            .uri("/rdevices/weather/forecast/2")
            .to_request();

        let response = test::call_service(&mut test, request).await;
        assert!(response.status().is_success());
    }
    #[actix_rt::test]
    async fn get_climate() {
        let mut test = test::init_service(
            App::new().service(
                web::scope("/rdevices")
                    .route("/weather/climate/{day}", web::get().to(get_fake_climate)),
            ),
        )
        .await;

        let request = test::TestRequest::get()
            .uri("/rdevices/weather/climate/2")
            .to_request();

        let response = test::call_service(&mut test, request).await;
        assert!(response.status().is_success());
    }

    #[actix_rt::test]
    async fn turn_on_switch() {
        let mut test = test::init_service(
            App::new().service(
                web::scope("/rwdevices")
                    .route("/fake_switch", web::post().to(turn_switch))
                    .route("/fake_switch", web::get().to(get_switch_status)),
            ),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/rwdevice/fake_switch")
            .set_json(&json!({"on":true}))
            .to_request();
        test::call_service(&mut test, request).await;
        let request = test::TestRequest::get()
            .uri("/rwdevices/fake_switch")
            .to_request();
        let response = test::call_service(&mut test, request).await;
        assert!(response.status().is_success());
    }

    #[actix_rt::test]
    async fn create_rdevice_() {
        let device = RDevice::new(
            "fake_devices",
            "fake_plugin",
            "./target/debug/libfake_plugin.so",
            "127.0.0.1",
        )
        .unwrap();

        let mut test = test::init_service(
            App::new().service(web::scope("/").route("/rdevices", web::post().to(create_rdevice))),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/rdevices")
            .set_json(&device)
            .to_request();
        let response = test::call_service(&mut test, request).await;

        assert!(response.status().is_success());
    }

    #[actix_rt::test]
    async fn get_rdevices_() {
        let device = RDevice::new(
            "fake_devices",
            "fake_plugin",
            "./target/debug/libfake_plugin.so",
            "127.0.0.1",
        )
        .unwrap();

        let mut test = test::init_service(
            App::new().service(
                web::scope("/")
                    .route("/rdevices", web::post().to(create_rdevice))
                    .route("/rdevices", web::get().to(get_rdevices)),
            ),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/rdevices")
            .set_json(&device)
            .to_request();
        test::call_service(&mut test, request).await;
        let request = test::TestRequest::get().uri("/rdevices").to_request();
        let response = test::call_service(&mut test, request).await;
        assert!(response.status().is_success());
    }

    #[actix_rt::test]
    async fn delete_rdevice_() {
        let device = RDevice::new(
            "fake_devices",
            "fake_plugin",
            "./target/debug/libfake_plugin.so",
            "127.0.0.1",
        )
        .unwrap();

        let mut test = test::init_service(
            App::new().service(
                web::scope("/")
                    .route("/rdevices", web::get().to(get_rdevices))
                    .route("/rdevices", web::post().to(create_rdevice))
                    .route("/rdevice/{device}", web::delete().to(delete_rdevice)),
            ),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/rdevices")
            .set_json(&device)
            .to_request();
        test::call_service(&mut test, request).await;
        let request = test::TestRequest::delete()
            .uri("/rdevice/fake_devices")
            .to_request();
        let response = test::call_service(&mut test, request).await;

        assert!(response.status().is_success());
    }

    #[actix_rt::test]
    async fn create_rwdevice_() {
        let device = RDevice::new(
            "fake_devices",
            "fake_plugin",
            "./target/debug/libfake_plugin.so",
            "127.0.0.1",
        )
        .unwrap();

        let mut test = test::init_service(
            App::new().service(web::scope("/").route("/rdevices", web::post().to(create_rdevice))),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/rdevices")
            .set_json(&device)
            .to_request();
        let response = test::call_service(&mut test, request).await;

        assert!(response.status().is_success());
    }

    #[actix_rt::test]
    async fn get_rwdevices_() {
        let device = RWDevice::new(
            "fake_devices",
            "fake_plugin",
            "./target/debug/libfake_plugin.so",
            "127.0.0.1",
        )
        .unwrap();

        let mut test = test::init_service(
            App::new().service(
                web::scope("/")
                    .route("/rwdevices", web::post().to(create_rwdevice))
                    .route("/rwdevices", web::get().to(get_rwdevices)),
            ),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/rwdevices")
            .set_json(&device)
            .to_request();
        test::call_service(&mut test, request).await;
        let request = test::TestRequest::get().uri("/rwdevices").to_request();
        let response = test::call_service(&mut test, request).await;
        assert!(response.status().is_success());
    }

    #[actix_rt::test]
    async fn delete_rwdevice_() {
        let device = RWDevice::new(
            "fake_devices",
            "fake_plugin",
            "./target/debug/libfake_plugin.so",
            "127.0.0.1",
        )
        .unwrap();

        let mut test = test::init_service(
            App::new().service(
                web::scope("/")
                    .route("/rwdevices", web::get().to(get_rwdevices))
                    .route("/rwdevices", web::post().to(create_rwdevice))
                    .route("/rwdevice/{device}", web::delete().to(delete_rwdevice)),
            ),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/rwdevices")
            .set_json(&device)
            .to_request();
        test::call_service(&mut test, request).await;
        let request = test::TestRequest::delete()
            .uri("/rwdevice/fake_devices")
            .to_request();
        let response = test::call_service(&mut test, request).await;

        assert!(response.status().is_success());
    }
}

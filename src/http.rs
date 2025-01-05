use crate::system_stats::SysUsage;

pub(crate) struct HttpResponse {
    pub(crate) status: u16,
    pub(crate) success: bool,
}

pub(crate) async fn post_system_stats(url:String, usage:SysUsage) -> HttpResponse {
    let json = serde_json::to_string(&usage).unwrap();

    let client = reqwest::Client::new();
    let res = client.post(url)
        .body(json)
        .header("Content-Type", "application/json")
        .send()
        .await;


    if res.is_err() {
        return HttpResponse {
            status: 0,
            success: false
        }
    }

    let res = res.unwrap();



    HttpResponse {
        status: res.status().as_u16(),
        success: res.status().is_success()
    }
}

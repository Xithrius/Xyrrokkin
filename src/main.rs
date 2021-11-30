use actix_web::{post, web, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use octocrab::models::{
    events::payload::{PullRequestEventAction, PullRequestEventPayload},
    Installation,
};
use reqwest::header;

mod handlers;
mod utils;

use crate::{
    handlers::app::App,
    utils::{diff::parse_diff, table::create_markdown_table},
};

#[actix_web::main]
async fn main() -> Result<()> {
    let app = App::new().await;

    let uri = app.config.server_uri.clone();

    HttpServer::new(move || {
        actix_web::App::new()
            .service(index)
            .app_data(web::Data::new(app.clone()))
    })
    .bind(&uri)
    .unwrap_or_else(|_| panic!("Failed to bind to URI {}", uri))
    .run()
    .await
    .unwrap();

    Ok(())
}

#[post("/")]
async fn index(
    response: web::Json<Box<PullRequestEventPayload>>,
    app: web::Data<App>,
) -> impl Responder {
    if let PullRequestEventAction::Closed = &response.action {
        if let Some(_merged_time) = response.pull_request.merged_at {
            // Get all the places that the app is installed.
            let installations: Box<Vec<Installation>> = app
                .client
                .get("https://api.github.com/app/installations")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            let repo_url = response.pull_request.url.split('/').collect::<Vec<&str>>();

            // Since `response.pull_request.repo` can be `None`, we need to gaurantee a repository owner.
            // PR URL format: https://api.github.com/repos/{owner}/{repo}/pulls/{number}
            if let Some(installation) = installations
                .iter()
                .find(|install| install.account.login == repo_url[repo_url.len() - 4])
            {
                let installation_token: octocrab::models::InstallationToken = app
                    .client
                    .post(format!(
                        "https://api.github.com/app/installations/{}/access_tokens",
                        &installation.id
                    ))
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                // Docs: https://docs.github.com/en/rest/reference/issues#create-an-issue-comment
                // URL format: https://api.github.com/repos/{owner}/{repo}/issues/{issue_number}/comments
                let url = format!(
                    "{}/issues/{}/comments",
                    &repo_url[..6].join("/"),
                    response.pull_request.number
                );

                let diff_raw = app
                    .client
                    .get(response.pull_request.url.to_string())
                    .header(header::ACCEPT, "application/vnd.github.v3.diff")
                    .header(header::AUTHORIZATION, app.json_web_token.to_string())
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                let table = create_markdown_table(
                    vec![
                        "Extension".to_string(),
                        "Total".to_string(),
                        "New".to_string(),
                        "Deleted".to_string(),
                    ],
                    parse_diff(diff_raw)
                        .iter()
                        .map(|v| v.to_vec())
                        .collect::<Vec<Vec<String>>>(),
                );

                let comment_response = app
                    .client
                    .post(&url)
                    .header(
                        header::AUTHORIZATION,
                        format!("token {}", installation_token.token),
                    )
                    .body(json::stringify(json::object! {"body": table}))
                    .send()
                    .await
                    .unwrap();

                if comment_response.status() != 201 {
                    log::error!(
                        "{}: Could not post comment to {}.",
                        comment_response.status(),
                        url
                    );
                }
            }
        }
    }

    HttpResponse::Ok()
}

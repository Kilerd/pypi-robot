#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;

use serde::{Deserialize, Serialize};
use unhtml::{self, FromHtml};
use unhtml_derive::FromHtml;

use std::{env, io};

use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
    Result,
};
use serde_with::skip_serializing_none;
use telegram_typing_bot::typing::{InlineKeyboardMarkup, ParseMode, UpdateMessage};

use once_cell::sync::Lazy;

#[skip_serializing_none]
#[derive(FromHtml, Debug)]
struct Package {
    #[html(selector = "span.package-snippet__name", attr = "inner")]
    name: String,
    #[html(selector = "span.package-snippet__version", attr = "inner")]
    version: String,
    #[html(selector = "p.package-snippet__description", attr = "inner")]
    description: String,
}

#[skip_serializing_none]
#[derive(FromHtml, Debug)]
struct PypiSearchResult {
    #[html(selector = "a.package-snippet")]
    pub packages: Vec<Package>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
struct AnswerInlineQuery {
    inline_query_id: String,
    results: Vec<InlineQueryResult>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum InlineQueryResult {
    InlineQueryResultArticle(InlineQueryResultArticle),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
struct InlineQueryResultArticle {
    r#type: String,
    id: String,
    title: String,
    input_message_content: InputMessageContent,
    reply_markup: Option<InlineKeyboardMarkup>,
    url: Option<String>,
    hide_url: Option<bool>,
    description: Option<String>,
    thumb_url: Option<String>,
    thumb_width: Option<i32>,
    thumb_height: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum InputMessageContent {
    InputTextMessageContent(InputTextMessageContent),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
struct InputTextMessageContent {
    message_text: String,
    parse_mode: Option<ParseMode>,
    disable_web_page_preview: Option<bool>,
}

static TELEGRAM_BOT_TOKEN: Lazy<String> =
    Lazy::new(|| std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set"));

#[post("")]
async fn telegram_webhook_handler(
    update: web::Json<telegram_typing_bot::typing::Update>,
) -> impl Responder {
    let x1 = update.into_inner();
    match x1.message {
        UpdateMessage::InlineQuery(msg) => actix_rt::spawn(async move {
            let string = surf::get(format!("https://pypi.org/search/?q={}", msg.query))
                .await
                .unwrap()
                .body_string()
                .await
                .unwrap();
            let result: PypiSearchResult = PypiSearchResult::from_html(&string).unwrap();

            let x: Vec<InlineQueryResult> = result
                .packages
                .iter()
                .map(|package| {
                    InlineQueryResult::InlineQueryResultArticle(InlineQueryResultArticle {
                        r#type: "article".to_string(),
                        id: package.name.clone(),
                        title: format!("{} {}", package.name.clone(), package.version.clone()),
                        input_message_content: InputMessageContent::InputTextMessageContent(
                            InputTextMessageContent {
                                message_text: format!(
                                    "{} ({})\n{}",
                                    package.name.clone(),
                                    package.version,
                                    package.description
                                ),
                                parse_mode: None,
                                disable_web_page_preview: Some(false),
                            },
                        ),
                        reply_markup: None,
                        url: None,
                        hide_url: None,
                        description: Some(package.description.clone()),
                        thumb_url: None,
                        thumb_width: None,
                        thumb_height: None,
                    })
                })
                .collect();
            let query = AnswerInlineQuery {
                inline_query_id: msg.id.to_string(),
                results: x,
            };

            let string1 = surf::post(format!(
                "https://api.telegram.org/bot{}/answerInlineQuery",
                *TELEGRAM_BOT_TOKEN
            ))
            .body_json(&query)
            .unwrap()
            .await
            .unwrap()
            .body_string()
            .await
            .unwrap();
            debug!(
                "send inline query answer, query={} answer={}",
                msg.query, string1
            );
        }),
        _ => {}
    };
    "True"
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "pypirobot=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(
                web::scope(&format!("/telegram/{}", *TELEGRAM_BOT_TOKEN))
                    .service(telegram_webhook_handler),
            )
    })
    .bind("0.0.0.0:8000")?
    .start()
    .await
}

use dotenv::dotenv;
use rand::Rng;
use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
};
use uuid::Uuid;

const TEST_TYPES: [[&str; 3]; 2] = [
    ["пидóра", "пидóр", "пидóр"],
    ["чепубелю", "чебупеля", "чебупеля"],
];

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Strating bot...");

    let bot = Bot::from_env();

    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        |bot: Bot, inline_query: InlineQuery| async move {
            let percentage = rand::rng().random_range(0..=100);

            let [target, message] = if inline_query.query.is_empty() {
                [String::from("ты"), String::from("Я")]
            } else {
                [inline_query.query.clone(), inline_query.query]
            };

            let results = TEST_TYPES.iter().map(|variant| {
                InlineQueryResult::Article(
                    InlineQueryResultArticle::new(
                        // Each item needs a unique ID, as well as the response container for the
                        // items. These can be whatever, as long as they don't
                        // conflict.
                        Uuid::new_v4().to_string(),
                        format!("Пройти тест на {}", variant[0]),
                        InputMessageContent::Text(InputMessageContentText::new(format!(
                            "Поздравьте! {} на {}% {}!",
                            message, percentage, variant[1]
                        ))),
                    )
                    .description(format!(
                        "Давай проверим, насколько {} {} :)",
                        target, variant[2]
                    )),
                )
            });

            let response = bot
                .answer_inline_query(inline_query.id.clone(), results)
                .send()
                .await;

            if let Err(err) = response {
                log::error!("Error in handler: {err:?}");
            }

            respond(())
        },
    ));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

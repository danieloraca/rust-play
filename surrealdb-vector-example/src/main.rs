use serde_json::Value;
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    DB.connect::<Ws>("localhost:8678").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("test").use_db("test").await?;

    DB.query(
        "DEFINE TABLE
        IF NOT EXISTS liquids SCHEMALESS
        PERMISSIONS FOR CREATE, SELECT
        WHERE $auth, FOR UPDATE, DELETE
        WHERE created_by = $auth;",
    )
    .await?
    .check()?;

    // Create an index on the 'vector' field for vector search
    DB.query("DEFINE ANALYZER IF NOT EXISTS liquid_analyzer TOKENIZERS blank,class,camel,punct FILTERS snowball(english);")
        .await?
        .check()?;

    DB.query("DEFINE INDEX IF NOT EXISTS liquid_content ON liquids FIELDS content SEARCH ANALYZER liquid_analyzer BM25 HIGHLIGHTS;")
        .await?
        .check()?;

    DB.query("INSERT INTO liquidsVector [
        {sample:'Sea water', content: 'The sea water contains some amount of lead', embedding: [0.1, 0.2, 0.3, 0.4] },
        {sample:'Tap water', content: 'The team lead by Dr. Rose found out that the tap water in was potable', embedding:[1.0, 0.1, 0.4, 0.3]},
        {sample:'Sewage water', content: 'High amounts of a were found in Sewage water', embedding : [0.4, 0.3, 0.2, 0.1]}
    ];")
        .await?
        .check()?;

    DB.query("DEFINE INDEX IF NOT EXISTS mt_pts ON liquidsVector FIELDS embedding MTREE DIMENSION 4 DIST COSINE TYPE F32;")
        .await?
        .check()?;

    let sample_vector = vec![0.15, 0.25, 0.35, 0.45];

    let mut search_results = DB.query(
        r#"
        SELECT sample, content, vector::similarity::cosine(embedding, $vector) AS dist FROM liquidsVector WHERE embedding <|2|> $vector;
        "#,
    )
    .bind(("vector", sample_vector.clone()))
    .await?
    .check()?;

    // println!("Search results: {:?}", search_results);

    let results: Vec<Value> = search_results.take(0)?;
    for (i, result) in results.iter().enumerate() {
        println!(
            "Result {}:\n  Sample: {}\n  Content: {}\n  Distance: {:.4}\n",
            i + 1,
            result["sample"],
            result["content"],
            result["dist"].as_f64().unwrap_or(0.0)
        );
    }

    Ok(())
}

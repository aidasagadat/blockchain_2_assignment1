use reqwest;
use serde::{Deserialize, Serialize};
use warp::{Filter, http::Method};
use std::error::Error as StdError;

#[derive(Debug, Serialize, Deserialize)]
struct NewsArticle {
    title: String,
    source: String,
    date: String,
    summary: String,
    url: String,
    image_url: String,
}

async fn fetch_crypto_news(crypto_name: &str) -> Result<Vec<NewsArticle>, Box<dyn StdError>> {
    let url = format!(
        "https://cryptonews-api.com/api/v1/category?section=general&items=3&token={}&search={}",
        "tn07ovdewdsh6znt6ar0ps0rb0h2kc1vvrmmgjxh", 
        crypto_name
    );

    let response = reqwest::get(&url).await?;
    let response_text = response.text().await?;
    println!("API Response: {}", response_text);

    let json: serde_json::Value = serde_json::from_str(&response_text)?;

    let articles: Vec<NewsArticle> = json["data"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|article| NewsArticle {
            title: article["title"].as_str().unwrap_or("No title available").to_string(),
            source: article["source_name"].as_str().unwrap_or("Unknown").to_string(),
            date: article["date"].as_str().unwrap_or("Unknown").to_string(),
            summary: article["text"].as_str().unwrap_or("No summary available").to_string(),
            url: article["news_url"].as_str().unwrap_or("").to_string(),
            image_url: article["image_url"].as_str().unwrap_or("").to_string(),
        })
        .collect();

    Ok(articles)
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()  
        .allow_method(Method::GET)  
        .allow_header("Content-Type");  

    let crypto_news = warp::path!("news")
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and_then(|params: std::collections::HashMap<String, String>| async move {
            let default_name = String::new();  
            let crypto_name = params.get("crypto").unwrap_or(&default_name);  

            match fetch_crypto_news(crypto_name).await {
                Ok(news) => Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&news)),
                Err(_) => Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json::<Vec<NewsArticle>>(&vec![])), // Return an empty vector in case of error
            }
        });

    let static_files = warp::fs::dir("public");

    let routes = static_files.or(crypto_news).with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000))
        .await;
}



// use reqwest;
// use serde::{Deserialize, Serialize};
// use warp::Filter;
// use std::error::Error as StdError;

// #[derive(Debug, Serialize, Deserialize)]
// struct NewsArticle {
//     title: String,
//     source: String,
//     date: String,
//     summary: String,
//     url: String,
//     image_url: String,
// }

// // Define the function to fetch crypto news
// async fn fetch_crypto_news(crypto_name: &str) -> Result<Vec<NewsArticle>, Box<dyn StdError>> {
//     let url = format!(
//         "https://cryptonews-api.com/api/v1/category?section=general&items=3&token={}&search={}",
//         "tn07ovdewdsh6znt6ar0ps0rb0h2kc1vvrmmgjxh", // Replace with your actual API key
//         crypto_name
//     );

//     let response = reqwest::get(&url).await?;
//     let response_text = response.text().await?;
//     println!("API Response: {}", response_text);

//     let json: serde_json::Value = serde_json::from_str(&response_text)?;

//     let articles: Vec<NewsArticle> = json["data"]
//         .as_array()
//         .unwrap_or(&vec![])
//         .iter()
//         .map(|article| NewsArticle {
//             title: article["title"].as_str().unwrap_or("No title available").to_string(),
//             source: article["source_name"].as_str().unwrap_or("Unknown").to_string(),
//             date: article["date"].as_str().unwrap_or("Unknown").to_string(),
//             summary: article["text"].as_str().unwrap_or("No summary available").to_string(),
//             url: article["news_url"].as_str().unwrap_or("").to_string(),
//             image_url: article["image_url"].as_str().unwrap_or("").to_string(),
//         })
//         .collect();

//     Ok(articles)
// }

// #[tokio::main]
// async fn main() {
//    // Route for fetching crypto news
//    let crypto_news = warp::path!("news")
//    .and(warp::get())
//    .and(warp::query::<std::collections::HashMap<String, String>>())
//    .and_then(|params: std::collections::HashMap<String, String>| async move {
//        let default_name = String::new();  // Create a long-lived value
//        let crypto_name = params.get("crypto").unwrap_or(&default_name);  // Borrow the long-lived value

//        // Perform the async operation and then return the result
//        match fetch_crypto_news(crypto_name).await {
//            Ok(news) => Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&news)),
//            Err(_) => Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json::<Vec<NewsArticle>>(&vec![])), // Return an empty vector in case of error
//        }
//    });


//     // Serve static files (HTML, CSS, JS)
//     let static_files = warp::fs::dir("public");

//     // Combine the routes
//     let routes = static_files.or(crypto_news);

//     // Start the server
//     warp::serve(routes)
//         .run(([127, 0, 0, 1], 8000))
//         .await;
// }




// use reqwest;
// use serde::{Deserialize, Serialize};
// use std::error::Error as StdError;

// // Define the struct to store article information
// #[derive(Debug)]
// struct NewsArticle {
//     title: String,
//     source: String,
//     date: String,
//     summary: String,
//     url: String,
//     image_url: String,
// }

// // Define the function to fetch crypto news
// async fn fetch_crypto_news(crypto_name: &str) -> Result<Vec<NewsArticle>, Box<dyn StdError>> {
//     // Construct the URL with the API key and cryptocurrency name
//     let url = format!(
//         "https://cryptonews-api.com/api/v1/category?section=general&items=3&token={}&search={}",
//         "tn07ovdewdsh6znt6ar0ps0rb0h2kc1vvrmmgjxh", // Replace with your actual API key
//         crypto_name
//     );

//     // Fetch the response from the API
//     let response = reqwest::get(&url).await?;
    
//     // Get the response text for debugging
//     let response_text = response.text().await?;
//     println!("API Response: {}", response_text);

//     // Parse the JSON response into a serde_json::Value
//     let json: serde_json::Value = serde_json::from_str(&response_text)?;

//     // Parse the JSON to create the NewsArticle objects
//     let articles: Vec<NewsArticle> = json["data"]
//         .as_array()
//         .unwrap_or(&vec![])
//         .iter()
//         .map(|article| NewsArticle {
//             title: article["title"].as_str().unwrap_or("No title available").to_string(),
//             source: article["source_name"].as_str().unwrap_or("Unknown").to_string(),
//             date: article["date"].as_str().unwrap_or("Unknown").to_string(),
//             summary: article["text"].as_str().unwrap_or("No summary available").to_string(),
//             url: article["news_url"].as_str().unwrap_or("").to_string(),
//             image_url: article["image_url"].as_str().unwrap_or("").to_string(), // Add image URL
//         })
//         .collect();

//     Ok(articles)
// }

// // Main function to fetch and display the news
// #[tokio::main]
// async fn main() {
//     // Input from the user (you can change this to ask for user input instead)
//     let crypto_name = "bitcoin"; // Example cryptocurrency name

//     // Fetch the news articles
//     let news_articles = fetch_crypto_news(crypto_name).await.unwrap();

//     // Display the fetched news articles
//     for article in news_articles {
//         println!("Title: {}", article.title);
//         println!("Source: {}", article.source);
//         println!("Date: {}", article.date);
//         println!("Summary: {}", article.summary);
//         println!("URL: {}", article.url);
//         println!("Image URL: {}", article.image_url); // Print the image URL
//         println!("-----------------------------");
//     }
// }







// use reqwest::Error;
// use serde::{Deserialize, Serialize};
// use tokio;
// use std::error::Error as StdError;

// // API Key for CryptoNews API
// const API_KEY: &str = "tn07ovdewdsh6znt6ar0ps0rb0h2kc1vvrmmgjxh";

// // Struct to hold the news data
// #[derive(Serialize, Deserialize, Debug)]
// struct NewsArticle {
//     title: String,
//     source: String,
//     date: String,
//     summary: String,
//     url: String,
// }

// // Function to fetch news from the CryptoNews API
// async fn fetch_crypto_news(crypto_name: &str) -> Result<Vec<NewsArticle>, Box<dyn StdError>> {
//     let url = format!(
//         "https://cryptonews-api.com/api/v1/category?section=general&items=3&token={}&search={}",
//         API_KEY, crypto_name
//     );
    
//     // Fetch the response from the API
//     let response = reqwest::get(&url).await?;
    
//     // Print the raw response text for debugging
//     let response_text = response.text().await?;
//     println!("API Response: {}", response_text);

//     let json: serde_json::Value = serde_json::from_str(&response_text)?;

//     // Check if the response contains any articles
//     let articles: Vec<NewsArticle> = json["data"]
//         .as_array()
//         .unwrap_or(&vec![])
//         .iter()
//         .map(|article| NewsArticle {
//             title: article["title"].as_str().unwrap_or("N/A").to_string(),
//             source: article["source"]["name"].as_str().unwrap_or("Unknown").to_string(),
//             date: article["published"].as_str().unwrap_or("Unknown").to_string(),
//             summary: article["description"].as_str().unwrap_or("No summary available").to_string(),
//             url: article["url"].as_str().unwrap_or("").to_string(),
//         })
//         .collect();

//     Ok(articles)
// }


// // Main function to fetch and display the news
// #[tokio::main]
// async fn main() {
//     println!("Enter a cryptocurrency name or symbol:");
    
//     let mut crypto_name = String::new();
//     std::io::stdin().read_line(&mut crypto_name).expect("Failed to read line");
//     let crypto_name = crypto_name.trim();

//     match fetch_crypto_news(crypto_name).await {
//         Ok(articles) => {
//             if articles.is_empty() {
//                 println!("No news found for '{}'", crypto_name);
//             } else {
//                 for article in articles {
//                     println!("\nTitle: {}", article.title);
//                     println!("Source: {}", article.source);
//                     println!("Date: {}", article.date);
//                     println!("Summary: {}", article.summary);
//                     println!("URL: {}", article.url);
//                     println!("-----------------------------");
//                 }
//             }
//         }
//         Err(e) => println!("Error fetching news: {}", e),
//     }
// }

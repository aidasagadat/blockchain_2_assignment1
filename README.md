# blockchain_2_assignment1
A Rust-based service to collect and display cryptocurrency news.






# Crypto News Aggregator
A Rust-based service that collects and displays the latest cryptocurrency news from multiple sources. Users can input a cryptocurrency name or symbol to retrieve recent news articles.







## Usage
1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/crypto-news-aggregator.git
   cd crypto-news-aggregator

   
Install dependencies and build the project:
cargo build

Run the backend server:
cargo run


Open the frontend:
Open the index.html file in your browser or use a live server extension in your IDE to serve the frontend.

Search for cryptocurrency news:
Enter the name or symbol of the cryptocurrency (e.g., bitcoin, ethereum) in the input box and click "Get News" to fetch the latest articles related to that cryptocurrency.



API Interaction:
You can also interact with the API directly via the endpoint:
GET http://localhost:8000/news?crypto=<crypto_name_or_symbol>
Replace <crypto_name_or_symbol> with the name or symbol of the cryptocurrency you're interested in.


DEMO
![Screenshot from 2025-04-06 01-15-09](https://github.com/user-attachments/assets/0578fce5-77c6-4248-8cf9-bc67b6a6b4da)

![Screenshot from 2025-04-06 01-15-15](https://github.com/user-attachments/assets/080a5b3a-ca2a-44f4-97bb-30f8bfaafed0)

![Screenshot from 2025-04-06 01-15-21](https://github.com/user-attachments/assets/859d7552-e0b3-47e5-b66c-81c249607d60)





Examples
Example 1: Fetching Bitcoin News
API Request:
GET http://localhost:8000/news?crypto=bitcoin
API Response:
[
  {
    "title": "Bitcoin hits new all-time high",
    "source": "Crypto News Source",
    "date": "2025-04-06",
    "summary": "Bitcoin has reached a new milestone, breaking all previous records...",
    "url": "https://news.example.com/bitcoin-all-time-high",
    "image_url": "https://example.com/image.jpg"
  },
  ...
]





Example 2: Fetching Ethereum News
API Request:

GET http://localhost:8000/news?crypto=ethereum
API Response:
[
  {
    "title": "Ethereum network upgrade released",
    "source": "Coin News",
    "date": "2025-04-06",
    "summary": "The Ethereum network has undergone a major upgrade, enhancing scalability...",
    "url": "https://news.example.com/ethereum-upgrade",
    "image_url": "https://example.com/image2.jpg"
  },
  ...
]






License
This project is licensed under the MIT License 




# Cryptocurrency News Aggregator

## Overview
The Cryptocurrency News Aggregator is a Rust-based web service that collects and displays the latest news articles related to various cryptocurrencies. Users can search for news by entering the name or symbol of a cryptocurrency, and the application retrieves recent articles from multiple sources.

## Features
- Search for news by cryptocurrency name or symbol.
- Fetch data from multiple APIs, including CryptQNews and CoinGecko.
- Display news articles in a structured format, including title, source, date, summary, and link.
- Handle errors and manage API rate limits.
- Simple web interface for user interaction.
- (Optional) Implement caching to reduce API calls.

## Technology Stack
- **Backend**: Rust
- **Frontend**: Basic HTML/CSS or Rust-based UI (e.g., Yew)
- **Data Sources**: CryptQNews API, CoinGecko API
- **Caching & Storage**: Redis, SQLite, PostgreSQL (optional)

## Installation
1. Clone the repository:
   ```
   git clone https://github.com/yourusername/crypto-news-aggregator.git
   ```
2. Navigate to the project directory:
   ```
   cd crypto-news-aggregator
   ```
3. Build the project:
   ```
   cargo build
   ```

## Usage
1. Run the application:
   ```
   cargo run
   ```
2. Open your web browser and navigate to `http://localhost:8000`.
3. Enter a cryptocurrency name or symbol in the search bar to retrieve the latest news articles.

## Examples
- Searching for "Bitcoin" will display the latest news articles related to Bitcoin.
- Searching for "ETH" will show news articles related to Ethereum.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.
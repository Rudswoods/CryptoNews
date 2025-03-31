document.addEventListener('DOMContentLoaded', function() {
    const searchForm = document.getElementById('search-form');
    const resultsContainer = document.getElementById('results');

    searchForm.addEventListener('submit', async function(event) {
        event.preventDefault();
        const cryptoInput = document.getElementById('crypto-input').value;
        if (cryptoInput) {
            await fetchNews(cryptoInput);
        }
    });

    async function fetchNews(crypto) {
        try {
            const response = await fetch(`/api/news?crypto=${encodeURIComponent(crypto)}`);
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            const newsData = await response.json();
            displayResults(newsData);
        } catch (error) {
            console.error('Error fetching news:', error);
            resultsContainer.innerHTML = '<p>Error fetching news. Please try again later.</p>';
        }
    }

    function displayResults(newsData) {
        resultsContainer.innerHTML = '';
        if (newsData.length === 0) {
            resultsContainer.innerHTML = '<p>No news found for this cryptocurrency.</p>';
            return;
        }
        newsData.forEach(article => {
            const articleElement = document.createElement('div');
            articleElement.classList.add('news-article');
            articleElement.innerHTML = `
                <h3><a href="${article.link}" target="_blank">${article.title}</a></h3>
                <p><strong>Source:</strong> ${article.source}</p>
                <p><strong>Date:</strong> ${new Date(article.date).toLocaleString()}</p>
                <p>${article.summary}</p>
            `;
            resultsContainer.appendChild(articleElement);
        });
    }
});
function loadBlogPosts() {
    const postsContainer = document.getElementById("posts");

    if (!postsContainer) {
        return;
    }

    fetch('posts.json')
        .then(response => response.json())
        .then(data => {
            data.posts.forEach(post => {
                const postDiv = document.createElement("div");
                postDiv.classList.add("post");

                const postTitle = document.createElement("h2");
                postTitle.classList.add("post-title");
                const postLink = document.createElement("a");
                postLink.href = post.url;
                postLink.textContent = post.title;
                postTitle.appendChild(postLink);

                const postDate = document.createElement("p");
                postDate.classList.add("post-date");
                postDate.textContent = new Date(post.date).toLocaleDateString();

                const postDescription = document.createElement("p");
                postDescription.classList.add("post-description");
                postDescription.textContent = post.description;

                postDiv.appendChild(postTitle);
                postDiv.appendChild(postDate);
                postDiv.appendChild(postDescription);

                postsContainer.appendChild(postDiv);
            });
        });
}

/// Handle toggling between light and dark mode
function handleTheme() {
    Promise.all([waitForElem('#dark-mode-toggle'), 
                 waitForElem('#dark-mode'), 
                 waitForElem('#prism-dark-theme'), 
                 waitForElem('#prism-light-theme')]).then(function(elements) {
        const darkModeToggle = elements[0]; 
        const darkModeStylesheet = elements[1]; 

        const prismDarkTheme = elements[2];
        const prismLightTheme = elements[3];

        function setDarkMode(enabled) {
            if (enabled) {
                darkModeStylesheet.media = '';
                prismDarkTheme.media = '';
                prismLightTheme.media = 'none';
            } else {
                darkModeStylesheet.media = 'none';
                prismDarkTheme.media = 'none';
                prismLightTheme.media = '';
            }
        }

        function toggleDarkMode() {
            const darkModeEnabled = darkModeStylesheet.media === '';
            setDarkMode(!darkModeEnabled);
        }

        // Set initial state based on user preference or localStorage
        const userPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        const savedPreference = localStorage.getItem('darkModeEnabled');
        const darkModeEnabled = savedPreference !== null ? JSON.parse(savedPreference) : userPrefersDark;
        setDarkMode(darkModeEnabled);

        // Handle toggle span click
        darkModeToggle.addEventListener('click', () => {
            toggleDarkMode();
            localStorage.setItem('darkModeEnabled', JSON.stringify(!darkModeEnabled));
        });
    });
}

document.querySelectorAll('pre code').forEach(function (codeBlock) {
    const lines = codeBlock.textContent.split('\n');
    const minIndent = Math.min(...lines.filter(line => line.trim()).map(line => line.search(/\S/)));

    codeBlock.textContent = lines.map(line => line.slice(minIndent)).join('\n').trim();
});

document.addEventListener("DOMContentLoaded", () => {
    loadBlogPosts();
    handleTheme();
});

function waitForElem(selector) {
    return new Promise(resolve => {
        if (document.querySelector(selector)) {
            console.log('element already exists: ' + selector);
            return resolve(document.querySelector(selector));
        }

        const observer = new MutationObserver(mutations => {
            if (document.querySelector(selector)) {
                console.log('found element: ' + selector);
                resolve(document.querySelector(selector));
                observer.disconnect();
            }
        });

        observer.observe(document.body, {
              childList: true
            , subtree: true
            , attributes: false
            , characterData: false
        });
    });
}

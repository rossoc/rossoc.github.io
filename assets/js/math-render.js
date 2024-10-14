let macros = {
    "\\RR": "\\mathbb{R}",
    "\\EE": "\\mathbb{E}",
    "\\PP": "\\mathbb{P}",
    "\\macron": "\\bar",
}

document.addEventListener("DOMContentLoaded", function() {
    renderMathInElement(document.body, {
        macros,
        delimiters: [
            { left: '$$', right: '$$', display: true },
            { left: '$', right: '$', display: false },
            { left: '\\(', right: '\\)', display: false },
            { left: '\\[', right: '\\]', display: true }
        ],
        throwOnError: false,
    });
});

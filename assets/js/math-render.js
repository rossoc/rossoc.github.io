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

    navbar_active_toggle()

    nav = document.getElementById("navbar")

    nav.addEventListener('mouseenter', () => {
        navbar_active_toggle()
    })

    nav.addEventListener('mouseleave', () => {
        navbar_active_toggle()
    })
})

function navbar_active_toggle() {
    const currentPath = window.location.pathname.replace(/\/index\.html$/, '').replace(/\/$/, '')
    const currentPage = currentPath.split('/').pop() || 'index'

    const navLinks = document.querySelectorAll('nav a');

    navLinks.forEach(link => {
        const linkPage = link.getAttribute('href')
        if (linkPage.replace(/\/$/, '') === currentPage.replace(/\/$/, '')) {
            link.classList.toggle('active')
        }
    })
}

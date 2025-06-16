document.addEventListener("DOMContentLoaded", () => {

    function toggleTheme() {
        const currentTheme = document.documentElement.getAttribute("data-theme");
        const newTheme = currentTheme === "dark" ? "light" : "dark";

        // Update theme attribute
        document.documentElement.setAttribute("data-theme", newTheme);
        localStorage.setItem("theme", newTheme);

        // Toggle all light/dark elements
        document.querySelectorAll(".light, .dark").forEach(el => {
            el.classList.toggle("light");
            el.classList.toggle("dark");
        });
    }

    function applyStoredTheme() {
        const storedTheme = localStorage.getItem("theme") ||
            (window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light");

        document.documentElement.setAttribute("data-theme", storedTheme);
    }

    applyStoredTheme();
    const button = document.getElementById("theme-button");
    button ? button.addEventListener("click", toggleTheme) : null
});

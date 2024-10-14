function applyDarkModeToImages() {
    const darkModeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const images = document.querySelectorAll('img');

    function updateImageFilters() {
        images.forEach(image => {
            if (darkModeMediaQuery.matches) {
                image.style.filter = 'invert(1) hue-rotate(180deg)';
            } else {
                image.style.filter = 'none';
            }
        });
    }

    updateImageFilters();
    darkModeMediaQuery.addEventListener('change', updateImageFilters);
}

document.addEventListener('DOMContentLoaded', applyDarkModeToImages);

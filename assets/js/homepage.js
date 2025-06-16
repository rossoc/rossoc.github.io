document.addEventListener('mousemove', (e) => {
    const card = document.getElementById('home-main')
    const rect = card.getBoundingClientRect();
    const x = (e.clientX - rect.left - rect.width / 2) / 40;
    const y = -(e.clientY - rect.top - rect.height / 2) / 40;

    card.style.transform = `perspective(1000px) rotateY(${x}deg) rotateX(${y}deg)`;
});

function scrollToTop() {
    window.scrollTo({
        top: 0,
        behavior: 'smooth'
    });
}

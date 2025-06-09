document.addEventListener('mousemove', (e) => {
    const card = document.getElementById('home-main')
    const rect = card.getBoundingClientRect();
    const x = (e.clientX - rect.left - rect.width / 2) / 30;
    const y = -(e.clientY - rect.top - rect.height / 2) / 30;

    card.style.transform = `perspective(1000px) rotateY(${x}deg) rotateX(${y}deg)`;
});

document.addEventListener("DOMContentLoaded", function() {
    if (getDeviceType() === "Desktop") {
        document.addEventListener('mousemove', (e) => {
            const card = document.getElementById('home-main')
            const rect = card.getBoundingClientRect();
            const x = (e.clientX - rect.left - rect.width / 2) / 40;
            const y = -(e.clientY - rect.top - rect.height / 2) / 40;

            card.style.transform = `perspective(1000px) rotateY(${x}deg) rotateX(${y}deg)`;
        })
    }

    const back_to_top = document.getElementById("back-to-top")
    back_to_top.addEventListener('mouseenter', () => {
        back_to_top_active_toggle(true)
    })
    back_to_top.addEventListener('mouseleave', () => {
        back_to_top_active_toggle(false)
    })
});

function back_to_top_active_toggle(flag) {
    const normal = document.getElementById('back-to-top-regular');
    const hover = document.getElementById('back-to-top-hover');

    flag ? normal.classList.add('hover') : normal.classList.remove('hover')
    flag ? hover.classList.remove('hover') : hover.classList.add('hover')
}

function scrollToTop() {
    window.scrollTo({
        top: 0,
        behavior: 'smooth'
    });
}

function getDeviceType() {
    const userAgent = navigator.userAgent;
    const width = window.innerWidth;
    if (/Mobi|Android/i.test(userAgent) || width <= 768) {
        return "Mobile";
    } else if (/Tablet|iPad/i.test(userAgent) || (width > 768 && width <= 1024)) {
        return "Tablet";
    } else {
        return "Desktop";
    }
}


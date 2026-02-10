const svgSource = `<svg xmlns="http://www.w3.org/2000/svg" width="500" height="500" fill="none" viewBox="0 0 500 500"><g filter="url(#a)" opacity=".8"><path fill="#ff71b3" fill-opacity=".7" d="M250 118.987c-21.828-36.393-58.207-61.868-101.862-61.868C86.293 57.12 39 104.43 39 166.297c0 120.096 65.483 138.292 211 276.584 145.517-138.292 211-156.488 211-276.584 0-61.867-47.293-109.178-109.138-109.178-43.655 0-80.034 25.475-101.862 61.868" shape-rendering="crispEdges"/></g><path fill="#ff71b3" fill-opacity=".7" d="M250 153.129c-16.144-26.908-43.051-45.744-75.338-45.744-45.742 0-80.72 34.981-80.72 80.725 0 88.798 48.432 102.253 156.058 204.505C357.626 290.363 406.058 276.908 406.058 188.11c0-45.744-34.978-80.725-80.72-80.725-32.287 0-59.194 18.836-75.338 45.744" opacity=".8"/><path fill="#e15192" fill-opacity=".4" d="M250 182.508c-11.246-18.747-29.99-31.871-52.483-31.871-31.865 0-56.232 24.372-56.232 56.243 0 61.868 33.739 71.242 108.715 142.483 74.976-71.241 108.715-80.615 108.715-142.483 0-31.871-24.367-56.243-56.232-56.243-22.493 0-41.237 13.124-52.483 31.871" opacity=".8"/><defs><filter id="a" width="485.125" height="448.886" x="7.438" y="25.557" color-interpolation-filters="sRGB" filterUnits="userSpaceOnUse"><feFlood flood-opacity="0" result="BackgroundImageFix"/><feColorMatrix in="SourceAlpha" result="hardAlpha" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"/><feOffset/><feGaussianBlur stdDeviation="15.781"/><feComposite in2="hardAlpha" operator="out"/><feColorMatrix values="0 0 0 0 0.93703 0 0 0 0 0.15389 0 0 0 0 0.493251 0 0 0 0.4 0"/><feBlend in2="BackgroundImageFix" result="effect1_dropShadow_1_2"/><feBlend in="SourceGraphic" in2="effect1_dropShadow_1_2" result="shape"/></filter></defs></svg>`

function makeHeart(px, py) {
    document.body.insertAdjacentHTML('beforeend', svgSource)
    const svg = document.body.lastElementChild
    
    const size = 10 + Math.random() * 30
    const x = px ?? (size + Math.random() * (window.innerWidth - size * 2))
    const y = py ?? (size + Math.random() * (window.innerHeight - size * 2))
    const cx = window.innerWidth / 2
    const cy = window.innerHeight / 2
    const r = Math.sqrt((x - cx) ** 2 + (y - cy) ** 2)
    const phi = Math.atan2(y - cy, x - cx)
    const r2 = r + 100
    const x2 = cx + r2 * Math.cos(phi)
    const y2 = cy + r2 * Math.sin(phi)
    
    Object.assign(svg.style, {
        position: 'fixed',
        height: size,
        width: size,
        transition: 'opacity 2000ms ease, transform 1500ms ease',
        zIndex: 99,
        pointerEvents: 'none',
    })
    
    svg.animate([
        { opacity: 0 },
        { opacity: 1 },
    ], {
        duration: 1000,
        easing: 'ease',
        fill: 'forwards',
    })
    
    svg.animate([
        { transform: 'scale(0.3)' },
        { transform: 'scale(1)' },
    ], {
        duration: 1500,
        easing: 'ease',
        fill: 'forwards',
    })
    
    svg.animate([
        { top: `${y - size / 2}px`, left: `${x - size / 2}px` },
        { top: `${y2 - size / 2}px`, left: `${x2 - size / 2}px` },
    ], {
        duration: 2000,
        delay: 150,
        easing: 'ease-in-out',
        fill: 'both',
    })
    
    svg.animate([
        { opacity: 1 },
        { opacity: 0 },
    ], {
        duration: 2000,
        delay: 1000,
        easing: 'ease',
        fill: 'forwards',
    })
    
    setTimeout(() => { svg.remove() }, 3500)
}

function makeMessage() {
    const div = document.body.appendChild(document.createElement('div'))
    div.innerText = 'Happy Valentine\'s Day!'
    
    const transform = 'translate(50%, -50%)'
    
    Object.assign(div.style, {
        width: 'fit-content',
        color: '#ffffff',
        background: '#ee4885',
        boxShadow: '0 0 15px 15px #ee4885',
        padding: '10px 25px',
        borderRadius: '20px',
        zIndex: 999,
        position: 'fixed',
        top: '50%',
        right: '50%',
        transform,
        userSelect: 'none',
        pointerEvents: 'none',
        textAlign: 'center',
    })
    
    div.animate([
        { opacity: 0 },
        { opacity: 1 },
    ], {
        duration: 1500,
        easing: 'ease',
        fill: 'forwards',
    })
    
    div.animate([
        { opacity: 1 },
        { opacity: 0 },
    ], {
        delay: 2500,
        duration: 1000,
        easing: 'ease',
        fill: 'forwards',
    })
    
    div.animate([
        { transform: `${transform} scale(0.8)` },
        { transform: `${transform} scale(1)` },
    ], {
        duration: 3500,
        easing: 'ease',
        fill: 'forwards',
    })
    
    setTimeout(() => { div.remove() }, 4000)
}

let interval = setInterval(() => makeHeart(), 100)

window.addEventListener('mouseover', (event) => {
    if (interval !== null) {
        setTimeout(() => makeMessage(), 250)
        const _interval = interval
        setTimeout(() => { clearInterval(_interval) }, 2000)
        interval = null
    }
    
    if (event.target.tagName === 'A') {
        makeHeart(event.clientX, event.clientY)
    }
})
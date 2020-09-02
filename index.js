const {Universe, Complex} = wasm_bindgen;

async function run() {
    wasm = await wasm_bindgen('./pkg/mbrot_bg.wasm');

    const canvas = document.getElementById('mbrot-canvas');
    const ctx = canvas.getContext('2d');

    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    const universe = Universe.new(canvas.width, canvas.height);
    var center = Complex.new(0.0, 0.0);
    var dx = 1.0/300;
    var max_iter = 1000;

    const draw = (center, dx, max_iter) => {
        universe.render(center, dx, max_iter);
        const pixels = new Uint8ClampedArray(wasm.memory.buffer,
                               universe.pixels(),
                               4*canvas.width*canvas.height);

        const image = new ImageData(pixels, canvas.width, canvas.height);
        ctx.putImageData(image, 0, 0);
    };

    const canvas_click = async function(e) {
        canvas.removeEventListener('click', canvas_click, false);
        const re = center.real() + dx * (e.clientX - canvas.width * 0.5);
        const img = center.imag() + dx * (e.clientY - canvas.height * 0.5);

        center = Complex.new(re, img);
        dx = dx * 0.25;

        draw(center, dx, max_iter);
        await new Promise(r => setTimeout(r, 1));
        canvas.addEventListener('click', canvas_click, false);
    };

    canvas.addEventListener('click', canvas_click, false);
    draw(center, dx, max_iter);
}
run();

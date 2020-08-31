import init, {Universe, Complex} from './pkg/mbrot.js';

async function run() {
    let wasm = await init();

    const m_canvas = document.getElementById('mbrot-canvas');
    const x_canvas = document.getElementById('ux-canvas');

    const m_ctx = m_canvas.getContext('2d');
    const x_ctx = x_canvas.getContext('2d');

    m_canvas.width = window.innerWidth;
    m_canvas.height = window.innerHeight;

    x_canvas.width = m_canvas.width;
    x_canvas.height = m_canvas.height;

    const universe = Universe.new(x_canvas.width, x_canvas.height);
    var center = Complex.new(0.0, 0.0);
    var dx = 1.0/300;
    var max_iter = 1000;

    var box = null;

    x_canvas.onmousedown = function(e) {
        if (box == null) {
            box = [e.clientX, e.clientY, 0, 0];
        }
    };

    x_canvas.onmousemove = function(e) {
        if (box != null) {
            x_ctx.lineWidth = 1;
            x_ctx.clearRect(0, 0, x_canvas.width, x_canvas.height);
            x_ctx.strokeStyle= '#FF3B03';
            box[2] = e.clientX;
            box[3] = e.clientY;
            x_ctx.strokeRect(box[0], box[1], box[2]-box[0], box[3]-box[1]);
        }
    };

    x_canvas.onmouseup = function(e) {
        const box_center_x = (box[0] + box[2]) / 2;
        const box_center_y = (box[1] + box[3]) / 2;
        const re = center.real() + dx * (box_center_x - m_canvas.width * 0.5);
        const img = center.imag() + dx * (box_center_y - m_canvas.height * 0.5);

        center = Complex.new(re, img);
        const x_rat = Math.abs(box[2] - box[0]) * 1.0 / m_canvas.width;
        const y_rat = Math.abs(box[3] - box[1]) * 1.0 / m_canvas.width;
        dx = dx * Math.max(x_rat, y_rat);

        universe.render(center, dx, max_iter);

        box = null;
        x_ctx.clearRect(0, 0, x_canvas.width, x_canvas.height);
    };

    const draw = () => {
        const pixelPtr = universe.pixels();
        const pixels = new Uint8ClampedArray(wasm.memory.buffer,
                               pixelPtr,
                               4*m_canvas.width*m_canvas.height);

        const image = new ImageData(pixels, m_canvas.width, m_canvas.height);
        m_ctx.putImageData(image, 0, 0);
    };

    const renderLoop = () => {
        draw();
        requestAnimationFrame(renderLoop);
    };

    universe.render(center, dx, max_iter);
    draw();
    renderLoop();
}
run();

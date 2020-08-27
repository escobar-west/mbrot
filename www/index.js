import {Universe, Complex} from "mbrot";
import {memory} from "mbrot/mbrot_bg";

const canvas = document.getElementById('mbrot-canvas');
const ctx = canvas.getContext('2d');
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

var box = null;

canvas.onmousedown = function(e) {
    if (box == null) {
        box = [e.clientX, e.clientY, 0, 0];
    }
}

canvas.onmousemove = function(e) {
    if (box != null) {
        ctx.lineWidth = 1;
        //ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.strokeStyle= '#FF3B03';
        box[2] = e.clientX;
        box[3] = e.clientY;
        ctx.strokeRect(box[0], box[1], box[2]-box[0], box[3]-box[1]);
    }
}

canvas.onmouseup = function(e) {
    box = null;
}

const universe = Universe.new(canvas.width, canvas.height);
const cval = Complex.new(0.377, 0.2);
const dx = 1.0/640;

universe.render(cval, dx, 500);

const renderLoop = () => {
    draw();
    requestAnimationFrame(renderLoop);
};

const draw = () => {
    const pixelPtr = universe.pixels();
    const pixels = new Uint8ClampedArray(memory.buffer,
                           pixelPtr,
                           4*canvas.width*canvas.height);

    const image = new ImageData(pixels, canvas.width, canvas.height);
    ctx.putImageData(image, 0, 0);
}

draw();
//renderLoop();

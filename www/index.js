import {Universe, Complex} from "mbrot";
import {memory} from "mbrot/mbrot_bg";

const canvas = document.getElementById('mbrot-canvas');
const ctx = canvas.getContext('2d');

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

const universe = Universe.new(canvas.width, canvas.height);
const cval = Complex.new(0.0,0.0);
universe.render(cval, 100);

const renderLoop = () => {
    draw();
    requestAnimationFrame(renderLoop);
};

const draw = () => {
    const pixelPtr = universe.pixels();
    const pixels = new Uint8ClampedArray(memory.buffer, pixelPtr, 4*canvas.width*canvas.height);
    const image = new ImageData(pixels, canvas.width, canvas.height);
    ctx.putImageData(image, 0, 0);
}

draw();
renderLoop();

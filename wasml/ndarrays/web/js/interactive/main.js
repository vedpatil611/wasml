import init, { Floats2d } from '../../../pkg/ndarrays.js';
import { createDisplayDiv } from './display.js';

(async () => {
  await init();
  const a = new Floats2d([
    [1, 2, 3],
    [4, 5, 6],
  ]);
  const b = new Floats2d([
    [7, 8, 9],
    [10, 11, 12],
  ]);

  const start = document.querySelector('section.start');
  start.appendChild(createDisplayDiv('a', a));
  start.appendChild(createDisplayDiv('b', b));
})();

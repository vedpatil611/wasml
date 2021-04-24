import init, { greet } from '../pkg/ndarrays.js';

(async () => {
  await init();

  greet();
})();

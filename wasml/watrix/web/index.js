import init, { greet } from '../pkg/web/watrix.js';

(async () => {
  await init();

  greet();
})();

import init, { greet } from '../pkg/dataframes.js';

(async () => {
  await init();

  greet();
})();

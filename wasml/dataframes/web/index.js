import init, { greet, SeriesF64 } from "../pkg/dataframes.js";

(async () => {
  await init();

  // greet();

  const exmaple = new SeriesF64("Dion", [1, 2, 3, 4]);
  console.table(exmaple.data());
})();

import init, { SeriesF64, SeriesI32 } from "../pkg/dataframes.js";

(async () => {
  await init();

  const int_data = new SeriesI32("Dion", [1, 2, 3, 4, 5]);
  // console.table(int_data.data());
  console.table(int_data.show());

  const float_data = new SeriesF64("pinto", [1.2, 6.9]);
  console.table(float_data.data());
})();

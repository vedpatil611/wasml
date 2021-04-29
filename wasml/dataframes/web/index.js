import init, { SeriesF64, SeriesI32 } from "../pkg/dataframes.js";

(async () => {
  await init();

  let int_data = new SeriesI32("Dion", [1, 2, 3, 4, 5]);
  // console.table(int_data.data());
  console.table(int_data.show());

  // console.log(int_data.data());
  // int_data.append(6);
  // console.log(int_data.appended(63939329));
  // int_data.extend([9, 8, 7]);
  // console.log(int_data.data());
  // console.log(int_data.extended([111, 111]));
  // console.log(int_data.data());
  const float_data = new SeriesF64("pinto", [1.2, 6.9]);
  console.table(float_data.show());
})();

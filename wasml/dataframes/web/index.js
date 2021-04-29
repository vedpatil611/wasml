import init, { SeriesF64, SeriesI32, DataFrame } from "../pkg/dataframes.js";

(async () => {
  await init();

  let ser1 = new SeriesI32("Dion", [1, 2, 3, 4, 5]);
  let ref_ser1 = ser1.get_ref();
  console.log("################--SERIES (Int)--##################");
  console.table(ser1.show());
  console.log("################--SERIES DATA--##################");
  console.table(ser1.data());

  let ser2 = new SeriesI32("Pinto", [6, 7, 8, 9]);
  let ref_ser2 = ser2.get_ref();
  console.log("################--SERIES (Float)--##################");
  const float_data = new SeriesF64("pinto", [1.2, 6.9]);
  console.table(float_data.show());

  let dataf = new DataFrame([ref_ser1, ref_ser2]);
  console.log("################--SERIES (Int)--##################");
  console.log(dataf.show());
})();

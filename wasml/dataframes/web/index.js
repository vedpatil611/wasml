import init, {
  SeriesF64,
  SeriesI32,
  DataFrame,
  ColumnType,
} from "../pkg/dataframes.js";

(async () => {
  await init();

  let ser1 = new SeriesI32("Dion", [1, 2]);
  let ref_ser1 = ser1.get_ref();
  console.log("################--SERIES (Int)--##################");
  console.table(ser1.show());
  console.log("################--SERIES DATA--##################");
  console.table(ser1.data());

  let ser2 = new SeriesI32("Pinto", [6, 7]);
  let ref_ser2 = ser2.get_ref();
  console.log("################--SERIES (Float)--##################");
  const float_data = new SeriesF64("pinto", [6, 6.9]);
  const float_data2 = new SeriesF64("p1into", [6.9, 6.9]);
  console.table(float_data.show());

  console.log(typeof ref_ser1);
  console.log(ref_ser1);

  let dataf = new DataFrame([ref_ser1, float_data2.get_ref(), ser2.get_ref()]);
  console.log("################--DF--##################");
  console.log(dataf.show());
  dataf.append(ColumnType.FLOAT, float_data.get_ref());
  console.log("################--DF--##################");
  console.log(dataf.show());

  console.log(dataf.size());
  console.log(dataf.columns());
  console.log(dataf.dTypes());
})();

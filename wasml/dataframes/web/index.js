import init, {
  SeriesF64,
  SeriesI32,
  DataFrame,
  ColumnType,
} from "../pkg/dataframes.js";

(async () => {
  await init();

  let ser1 = new SeriesI32("Dion", [1, 2]);
  let ref_ser1 = ser1.toJson();
  console.log("################--SERIES (Int)--##################");
  console.table(ser1.display);
  console.log("################--SERIES DATA--##################");
  console.table(ser1.data());

  let ser2 = new SeriesI32("Pinto", [6, 7]);
  let ref_ser2 = ser2.toJson();
  console.log("################--SERIES (Float)--##################");
  const float_data = new SeriesF64("Temp", [12, 12.9]);
  const float_data2 = new SeriesF64("Temp2", [9.99, 9.1]);

  float_data.extend([11.1, 11.1]);

  float_data2.extend([11.1, 11.1]);

  console.table(float_data.display);
  console.table(float_data2.display);

  let dataf = new DataFrame([ser1.get_ref(), float_data2.get_ref()]);

  let dataf = new DataFrame([ref_ser1, float_data2.toJson(), ser2.toJson()]);
  console.log("################--DF--##################");
  console.log(dataf.show());
  dataf.append(ColumnType.FLOAT, float_data.toJson());
  console.log("################--DF--##################");
  console.log(dataf.display);

  console.log(dataf.size());
  console.log(dataf.columns());
  console.log(dataf.dTypes);
})();

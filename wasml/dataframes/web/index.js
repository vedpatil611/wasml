import init, {
  SeriesF64,
  SeriesI32,
  DataFrame,
  ColumnType,
} from "../pkg/dataframes.js";

(async () => {
  await init();

  let ser1 = new SeriesI32("Day", [1, 2]);

  ser1.extend([3, 4]);

  console.log("################--SERIES (Int)--##################");
  console.table(ser1.display);
  console.log("################--SERIES DATA--##################");
  console.table(ser1.data());

  console.log("################--SERIES (Float)--##################");
  const float_data = new SeriesF64("Temp", [12, 12.9]);
  const float_data2 = new SeriesF64("Temp2", [9.99, 9.1]);

  float_data.extend([11.1, 11.1]);

  float_data2.extend([11.1, 11.1]);

  console.table(float_data.display);
  console.table(float_data2.display);

  let dataf = new DataFrame([ser1.get_ref(), float_data2.get_ref()]);

  console.log("################--DF--##################");
  console.log(dataf.display);
  dataf.append(ColumnType.FLOAT, float_data.get_ref());
  console.log("################--DF--##################");
  console.log(dataf.display);

  console.log(dataf.size());
  console.log(dataf.columns());
  console.log(dataf.dTypes);
})();

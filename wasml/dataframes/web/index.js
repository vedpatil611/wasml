import init, {
  SeriesF64,
  SeriesI32,
  DataFrame,
  ColumnType,
} from "../pkg/dataframes.js";

(async () => {
  await init();

  let ser1 = new SeriesI32("Integer", [1, 2, 5, 6, 7, 7, 882112]);

  // const float_data = new SeriesF64("Float", [12, 12.9]);

  // let dataf = new DataFrame([ser1.toJson(), float_data.toJson()]);
  // console.log("################--DF--##################");
  // console.log(dataf.display);
  console.table(ser1.display);
  // dataf.append(ColumnType.FLOAT, float_data.toJson());
  // console.log("################--DF--##################");
  // console.log(dataf.display);

  // console.log(dataf.size());
  // console.log(dataf.columns());
  // console.log(dataf.dTypes);

  // console.log(ser1.reversed().data);
})();

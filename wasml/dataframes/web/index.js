import init, {
  SeriesF64,
  SeriesI32,
  DataFrame,
  ColumnType,
} from "../pkg/dataframes.js";

(async () => {
  await init();

  let ser1 = new SeriesI32("Integer", [1, 2, 5, 6, 7, 8, 9]);
  console.log(ser1.display);

  let f1 = new SeriesF64("Floty", [1.11, 2.34]);
  console.log(f1.display);

  // let df = new DataFrame([ser1.toJson(), f1.toJson()]);
  // console.log(df.display);
})();

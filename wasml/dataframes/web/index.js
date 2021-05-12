import init, {
  SeriesF64,
  SeriesI32,
  DataFrame,
  ColumnType,
  SeriesSTR,
} from "../pkg/dataframes.js";

(async () => {
  await init();

  let ser1 = new SeriesI32("Integerdasd", [1, 2]);
  console.log(ser1.display);

  let ser2 = new SeriesI32("yolyoyl", [5, 6]);

  let f1 = new SeriesF64("Floty", [1.11, 2.34]);
  console.log(f1.display);

  let df1 = new DataFrame([ser1.toJson(), f1.toJson(), ser2.toJson()]);
  // console.log(df1.display);
  // console.log(df1.ilocr(1));
  console.log(df1.mean());

  // let s1 = new SeriesSTR("Arpit", ["tolyoy", "owkdos", "oapsoapsos"]);
  // console.log(s1.display);
  // console.log(s1.len());

  // let df = new DataFrame([ser1.toJson(), f1.toJson()]);
  // console.log(df.display);
})();

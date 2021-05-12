import init, {
  SeriesF64,
  SeriesI32,
  DataFrame,
  ColumnType,
  SeriesSTR,
} from "../pkg/dataframes.js";

(async () => {
  await init();

  // let ser1 = new SeriesI32("Integer", [1, 2]);
  // console.log(ser1.display);

  // let f1 = new SeriesF64("Floty", [1.11, 2.34]);
  // console.log(f1.display);

  // let df1 = new DataFrame([ser1.toJson(), f1.toJson()]);
  // console.log(df1.loc("Floty"));

  // let df = new DataFrame([ser1.toJson(), f1.toJson()]);
  // console.log(df.display);


  
  // let str1 = new SeriesSTR("Gay", ["ara", "Bata", "Chanaro", "Dio"]);
  // console.log(`str1`, str1.display);
  // console.log(`str1.get(2)`, str1.get(2))

  // str1.set(1,"meow");
  // console.log(`set(1,"meow")`, str1.display);

  // str1.swap( 1, 2);
  // console.log(`swap( 1, 2)`, str1.display);
  // str1.append("Nya");
  // console.log(`append("Nya")`, str1.display);
  
  // str1.splice(0);
  // console.log('splice(1)', str1.display)

  // str1.extend(["Hay", "Zeke"]);
  // console.log(`str1.extend(str1)`, str1.display)

  // str1.insert(1, "Hex");
  // console.log(`insert(1, "Hex")`, str1.display)
})();

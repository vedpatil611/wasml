import init, { Integers1d, Floats1d, Strings1d } from '../pkg/matrices.js';

(async () => {
  await init();

  // Integer stuff
  const int1d1 = new Integers1d([1, 2, 3, 4, 5, 6]);
  const int1d2 = new Integers1d([7, 8, 9, 10, 11, 12]);
  const intadd = int1d1.add(int1d2);
  const intsub = int1d1.sub(int1d2);
  const intmul = int1d1.mul(int1d2);
  const intdiv = int1d1.div(int1d2);

  console.log(int1d1.toString());
  console.log(int1d2.toString());
  console.log(intadd.data);
  console.log(intsub.data);
  console.log(intmul.data);
  console.log(intdiv.data);

  // Float stuff
  const float1d1 = new Floats1d([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
  const float1d2 = new Floats1d([7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
  const floatadd = float1d1.add(float1d2);
  const floatsub = float1d1.sub(float1d2);
  const floatmul = float1d1.mul(float1d2);
  const floatdiv = float1d1.div(float1d2);

  console.log(float1d1.toString());
  console.log(float1d2.toString());
  console.log(floatadd.data);
  console.log(floatsub.data);
  console.log(floatmul.data);
  console.log(floatdiv.data);

  // String stuff
  const str1d1 = new Strings1d(['a', 'b', 'c']);
  const str1d2 = new Strings1d(['d', 'e', 'f']);

  console.log(str1d1.data);
  console.log(str1d2.data);
})();

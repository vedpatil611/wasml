import init, { Integers1d, Floats1d, Strings1d } from '../pkg/ndarrays.js';

(async () => {
  await init();

  one_dimensional_integers();
  one_dimensional_floats();
  one_dimensional_strings();
})();

const one_dimensional_integers = () => {
  console.group('ONE DIMENSIONAL INTEGERS');

  const a = new Integers1d([1, 2, 3, 4, 5, 6]);
  const b = new Integers1d([7, 8, 9, 10, 11, 12]);

  const add = a.add(b);
  const sub = a.sub(b);
  const mul = a.mul(b);
  const div = a.div(b);

  console.log(a.toString());
  console.log(b.toString());
  console.log(add.data);
  console.log(sub.data);
  console.log(mul.data);
  console.log(div.data);

  console.groupEnd();
};

const one_dimensional_floats = () => {
  console.group('ONE DIMENSIONAL FLOATS');

  const a = new Floats1d([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
  const b = new Floats1d([7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);

  const add = a.add(b);
  const sub = a.sub(b);
  const mul = a.mul(b);
  const div = a.div(b);

  console.log(a.toString());
  console.log(b.toString());
  console.log(add.data);
  console.log(sub.data);
  console.log(mul.data);
  console.log(div.data);

  console.groupEnd();
};

const one_dimensional_strings = () => {
  console.group('ONE DIMENSIONAL STRINGS');

  const a = new Strings1d(['a', 'b', 'c']);
  const b = new Strings1d(['d', 'e', 'f']);

  console.log(a.data);
  console.log(b.data);

  console.groupEnd();
};

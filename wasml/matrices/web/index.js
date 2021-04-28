import init, { Integers1d, Floats1d, Strings1d } from '../pkg/ndarrays.js';

(async () => {
  await init();

  console.group(
    '%cONE DIMENSIONAL',
    'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
  );
  one_dimensional_integers();
  one_dimensional_floats();
  one_dimensional_strings();
  console.groupEnd();

  console.group(
    '%cTWO DIMENSIONAL',
    'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
  );
  console.groupEnd();
})();

const one_dimensional_integers = () => {
  console.group('INTEGERS');

  const a = new Integers1d([5, 2, 3, 4, 1]);
  const b = new Integers1d([6, 7, 8, 9, 10]);
  console.log(a.toString());
  console.log(b.toString());

  console.group('basic');
  console.log('a.get(0):', a.get(0));
  console.log('a.len():', a.len());
  console.log('a.shape()', a.shape());
  a.swap(0, 4);
  console.log('a after swap', a.data);
  console.groupEnd();

  console.group('math');
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

  console.groupEnd();
};

const one_dimensional_floats = () => {
  console.group('FLOATS');

  const a = new Floats1d([5.0, 2.0, 3.0, 4.0, 1.0]);
  const b = new Floats1d([6.0, 7.0, 8.0, 9.0, 10.0]);
  console.log(a.toString());
  console.log(b.toString());

  console.group('basic');
  console.log('a.get(0):', a.get(0));
  console.log('a.len():', a.len());
  console.log('a.shape()', a.shape());
  a.swap(0, 4);
  console.log('a after swap', a.data);
  console.groupEnd();

  console.group('math');
  const add = a.add(b);
  const sub = a.sub(b);
  const mul = a.mul(b);
  const div = a.div(b);

  console.log(add.data);
  console.log(sub.data);
  console.log(mul.data);
  console.log(div.data);
  console.groupEnd();

  console.groupEnd();
};

const one_dimensional_strings = () => {
  console.group('STRINGS');

  const a = new Strings1d(['e', 'b', 'c', 'd', 'a']);
  const b = new Strings1d(['f', 'g', 'h', 'i', 'j']);
  console.log(a.toString());
  console.log(b.toString());

  console.group('basic');
  console.log('a.get(0):', a.get(0));
  console.log('a.len():', a.len());
  console.log('a.shape()', a.shape());
  a.swap(0, 4);
  console.log('a after swap', a.data);
  console.groupEnd();

  console.group('math');
  console.log('no math operations on strings');
  console.groupEnd();

  console.groupEnd();
};

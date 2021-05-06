import init, { Floats1d, Floats2d, Strings1d } from '../pkg/ndarrays.js';
import { matrixMultiplicationTest, timeit } from './matrix_multiplication.js';

(async () => {
  await init();

  // matrixMultiplicationTest();
  // demo();

  console.group(
    '%cONE DIMENSIONAL',
    'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
  );
  one_dimensional_floats();
  one_dimensional_strings();

  console.groupEnd();

  console.group(
    '%cTWO DIMENSIONAL',
    'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
  );
  two_dimensional_floats();
  console.groupEnd();
})();

const demo = () => {};

const one_dimensional_floats = () => {
  console.group('FLOATS');

  one_dimensional_floats_basics();
  one_dimensional_floats_math();

  console.groupEnd();
};

const one_dimensional_strings = () => {
  console.group('STRINGS');
  one_dimensional_strings_basics();
  console.groupEnd();
}

const one_dimensional_strings_basics = () => {
  console.group('Basics');
  const s1 = new Strings1d(['a','b','c','d','e']);
  const s2 = new Strings1d(['p','q','r','s','t']);

  console.log('s1.len():', s1.len());
  console.log('s1.get(2):', s1.get(2));
  console.log('s2.get(1):', s2.get(1));
  console.log('s1.shape()', s1.shape());

  s2.swap(0, 4);
  console.log('s2.swap(0, 4)', s2.data);
  s1.set(2, 'x');
  console.log('s1.set(2, 10.0)', s1.data);
  
  console.log('s2.reversed()', s2.reversed().data);
  console.log('s2.appended("z")', s2.appended('z').data);
  console.log('s1.extended(s2)', s1.extended(s2).data);
  console.log('s1.inserted(3, "y")', s1.inserted(3, "y").data);
  console.log('s1.spliced(0)', s1.spliced(0)[0].data, s1.spliced(0)[1]);
  console.groupEnd();

}

const one_dimensional_floats_basics = () => {
  console.group('basics');

  const a = new Floats1d([5.0, 2.0, 10.0, 4.0, 1.0]);
  const b = new Floats1d([6.0, 7.0, 8.0, 9.0, 10.0]);

  console.log(a.toString());

  console.log('a.len():', a.len());
  console.log('a.shape()', a.shape());
  console.log('a.get(2):', a.get(2));
  a.set(2, 3.0);
  console.log('a.set(2, 10.0)', a.data);
  a.swap(0, 4);
  console.log('a.swap(0, 4)', a.data);
  console.log('a.reversed()', a.reversed().data);
  console.log('a.appended(6.0)', a.appended(6.0).data);
  console.log('a.extended(a)', a.extended(b).data);
  console.log('a.inserted(0, 0.0)', a.inserted(0, 0.0).data);
  console.log('a.spliced(0)', a.spliced(0)[0].data, a.spliced(0)[1]);

  console.groupEnd();
};

const one_dimensional_floats_math = () => {
  console.group('math');

  const a = new Floats1d([1.0, 2.0, 3.0, 4.0, 5.0]);
  const b = new Floats1d([6.0, 7.0, 8.0, 9.0, 10.0]);
  console.log(a.data);
  console.log(b.data);

  const add = a.add(b);
  const sub = a.sub(b);
  const mul = a.mul(b);
  const div = a.div(b);
  console.log('add', add.data);
  console.log('sub', sub.data);
  console.log('mul', mul.data);
  console.log('div', div.data);

  const a_clone = a.clone();
  a_clone.scaled_add(2, b);
  console.log('a.clone().scaled_add(2, b)', a_clone.data);
  console.log('a.sum()', a.sum());
  console.log('a.product()', a.product());
  console.log('a.variance(0)', a.variance(0));
  console.log('a.standard_deviation(0)', a.standard_deviation(0));
  console.log('a.variance(1)', a.variance(1));
  console.log('a.standard_deviation(1)', a.standard_deviation(1));

  console.groupEnd();
};

const two_dimensional_floats = () => {
  console.group('FLOATS');

  const a = new Floats2d([
    [1.0, 2.0],
    [4.0, 5.0],
  ]);
  const b = new Floats2d([
    [6.0, 7.0],
    [9.0, 10.0],
  ]);
  console.log(a.data);
  console.log(b.data);

  const add = a.add(b);
  const sub = a.sub(b);
  const mul = a.mul(b);
  const div = a.div(b);
  const dot = a.dot(b);
  console.log('add', add.data);
  console.log('sub', sub.data);
  console.log('mul', mul.data);
  console.log('div', div.data);
  console.log('dot', dot.data);

  console.groupEnd();
};

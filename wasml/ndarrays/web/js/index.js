import init, { Floats1d, Floats2d, Strings1d } from '../../pkg/ndarrays.js';
import { matrixMultiplicationTest, timeit } from './matrix_multiplication.js';

(async () => {
  await init();

  // demo();

  // console.group(
  //   '%cMATRIX MULTPLICATION TEST',
  //   'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
  // );
  // matmul(100);
  // console.groupEnd();

  // console.group(
  //   '%cONE DIMENSIONAL',
  //   'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
  // );
  // one_dimensional_floats();
  // console.groupEnd();

  console.group(
    '%cTWO DIMENSIONAL',
    'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
  );
  two_dimensional_floats();
  console.groupEnd();
})();

const demo = () => {};

const matmul = n => {
  console.log(`Multiplying 10 ${n}x${n} matrices ->`);
  const naive = [];
  const wasml = [];
  for (let i = 0; i < 10; ++i) {
    const [ntime, wtime] = matrixMultiplicationTest(n, n);
    naive.push(ntime);
    wasml.push(wtime);
  }
  const naive_avg = naive.reduce((a, b) => a + b) / naive.length;
  const naive_max = Math.max(...naive);
  const naive_min = Math.min(...naive);
  const wasml_avg = wasml.reduce((a, b) => a + b) / wasml.length;
  const wasml_max = Math.max(...wasml);
  const wasml_min = Math.min(...wasml);

  console.table({
    Naive: {
      Maximum: naive_max,
      Minimum: naive_min,
      Average: naive_avg,
    },
    WASML: {
      Maximum: wasml_max,
      Minimum: wasml_min,
      Average: wasml_avg,
    },
  });
};

const one_dimensional_floats = () => {
  console.group('FLOATS');

  one_dimensional_floats_basics();
  one_dimensional_floats_math();

  console.groupEnd();
};

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

  two_dimensional_floats_basics();
  two_dimensional_floats_math();

  console.groupEnd();
};

const two_dimensional_floats_basics = () => {
  console.group('basics');

  const a = new Floats2d([
    [1.0, 2.0, 3.0],
    [4.0, 6.0, 9.0],
  ]);

  console.log(a.toString());

  console.log('a.shape()', a.shape());
  console.log('a.get([2, 3])', a.get([1, 2]));
  a.set([1, 2], 5.0);
  console.log('a.set([2, 3], 5.0)', a.data);
  a.swap([1, 1], [1, 2]);
  console.log('a.swap([2, 2], [2, 3])', a.data);
  console.log('a.get_column(0)', a.get_row(0).data);
  console.log('a.get_row(0)', a.get_row(0).data);

  const a_clone = a.clone();
  const row = new Floats1d([-1.0, -2.0, -3.0]);
  a_clone.set_row(0, row);
  console.log('a_clone after set_row', a_clone.data);
  const col = new Floats1d([-4.0, 5.0]);
  a_clone.set_column(0, col);
  console.log('a_clone after set_column', a_clone.data);

  console.log(
    'a.row_appended(a.get_row(0))',
    a.row_appended(a.get_row(0)).data
  );
  console.log(
    'a.column_appended(a.get_column(0))',
    a.column_appended(a.get_column(0)).data
  );
  console.log('a.rows_extended(a.clone())', a.rows_extended(a.clone()).data);
  console.log(
    'a.columns_extended(a.clone())',
    a.columns_extended(a.clone()).data
  );

  console.log(
    'a.row_spliced(1)',
    a.row_spliced(1).map(x => x.data)
  );
  console.log(
    'a.column_spliced(1)',
    a.column_spliced(1).map(x => x.data)
  );

  console.groupEnd();
};

const two_dimensional_floats_math = () => {
  console.group('math');

  const a = new Floats2d([
    [1.0, 2.0, 3.0],
    [4.0, 5.0, 6.0],
  ]);
  const b = new Floats2d([
    [7.0, 8.0, 9.0],
    [10.0, 11.0, 12.0],
  ]);
  console.log(a.data);
  console.log(b.data);

  console.log('a.add(b)', a.add(b).data);
  console.log('a.sub(b)', a.sub(b).data);
  console.log('a.mul(b)', a.mul(b).data);
  console.log('a.div(b)', a.div(b).data);
  console.log('a.dot(b.transposed())', a.dot(b.transposed()).data);

  console.log('mean', a.mean());
  console.log('row mean', a.row_mean().data);
  console.log('column mean', a.column_mean().data);
  console.log('variance', a.variance(1));
  console.log('row variance', a.row_variance(1).data);
  console.log('column variance', a.column_variance(1).data);
  console.log('standard deviation', a.standard_deviation(1));
  console.log('row standard deviation', a.row_standard_deviation(1).data);
  console.log('column standard deviation', a.column_standard_deviation(1).data);

  console.groupEnd();
};

import init, { Floats2d } from '../../pkg/ndarrays.js';

export function timeit(callback) {
  const bef = new Date();
  const ret = callback();
  const aft = new Date();

  return [ret, aft - bef];
}

export function matrixMultiplicationTest(size) {
  const a = generateMatrix(size, size);
  const b = generateMatrix(size, size);

  const x = new Floats2d(a);
  const y = new Floats2d(b);

  const [vRes, vTime] = timeit(() => vanillaMultiplication(a, b));
  const [oRes, oTime] = timeit(() => x.dot(y));

  // console.log(
  //   `Time for pure js implementation: ${vTime.toString().padStart(8)} ms`
  // );
  // console.log(
  //   `Time for our implementation:     ${oTime.toString().padStart(8)} ms`
  // );

  return [vTime, oTime];
}

function generateMatrix(m, n) {
  let mat = new Array(m);
  for (let i = 0; i < m; i++) {
    mat[i] = new Array(n);
    for (let j = 0; j < n; j++) {
      mat[i][j] = Math.random() * 100;
    }
  }
  return mat;
}

function vanillaMultiplication(a, b) {
  let m = a.length,
    n = a[0].length;
  let p = b.length,
    q = b[0].length;
  let res = new Array(m);

  for (let i = 0; i < m; i++) {
    res[i] = new Array(n);
    for (let j = 0; j < q; j++) {
      let sum = 0;
      for (let k = 0; k < n; k++) {
        sum += a[i][k] * b[k][j];
      }
      res[i][j] = sum;
    }
  }
  return res;
}

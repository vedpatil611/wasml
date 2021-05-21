import { Floats1d, Floats2d } from '../../../pkg/ndarrays.js';

export const two_dimensional_floats = () => {
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
    console.log('a.getCol(0)', a.getCol(0).data);
    console.log('a.getRow(0)', a.getRow(0).data);

    const aClone = a.clone();
    const row = new Floats1d([-1.0, -2.0, -3.0]);
    aClone.setRow(0, row);
    console.log('aClone after setRow', aClone.data);
    const col = new Floats1d([-4.0, 5.0]);
    aClone.setCol(0, col);
    console.log('aClone after setCol', aClone.data);

    console.log('a.rowAppended(a.getRow(0))', a.rowAppended(a.getRow(0)).data);
    console.log('a.colAppended(a.getCol(0))', a.colAppended(a.getCol(0)).data);
    console.log('a.rowsExtended(a.clone())', a.rowsExtended(a.clone()).data);
    console.log('a.colsExtended(a.clone())', a.colsExtended(a.clone()).data);

    console.log('a.rowInserted(2, a.getRow(0))', a.rowInserted(2, a.getRow(0)).data);
    console.log('a.colInserted(2, a.getCol(0))', a.colInserted(2, a.getCol(0)).data);
    console.log(
        'a.rowSpliced(1)',
        a.rowSpliced(1).map(x => x.data)
    );
    console.log(
        'a.colSpliced(1)',
        a.colSpliced(1).map(x => x.data)
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
    console.log('row mean', a.rowMean().data);
    console.log('column mean', a.colMean().data);
    console.log('variance', a.variance(1));
    console.log('row variance', a.rowVariance(1).data);
    console.log('standard deviation', a.standardDeviation(1));
    console.log('column standard deviation', a.colStandardDeviation(1).data);

    console.groupEnd();
};

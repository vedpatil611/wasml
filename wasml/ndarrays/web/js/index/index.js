import init, { initThreadPool } from '../../../pkg/ndarrays.js';

import { matmul, matrixMultiplicationTest } from './matmul.js';
// import { one_dimensional_floats } from './one.js';
// import { two_dimensional_floats } from './two.js';

(async () => {
    await init();
    await initThreadPool(navigator.hardwareConcurrency);

    // console.group(
    //     '%cMATRIX MULTPLICATION TEST',
    //     'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
    // );
    // const data = {
    //     'size': [],
    //     'time': [],
    //     'type': [],
    // };
    // for (let i = 100; i <= 1000; i += 10) {
    //     try {
    //         for (let j = 0; j < 10; ++j) {
    //             const [ntime, wtime] = matrixMultiplicationTest(i, i);
    //             data['size'].push(i);
    //             data['time'].push(ntime);
    //             data['type'].push('naive');

    //             data['size'].push(i);
    //             data['time'].push(wtime);
    //             data['type'].push('wasml');
    //         }
    //         console.log(i, 'done');
    //     } catch (e) {
    //         console.error(e);
    //     }
    // }
    // console.log(data);
    // console.log(JSON.stringify(data));

    // console.groupEnd();

    // console.group(
    //     '%cONE DIMENSIONAL',
    //     'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
    // );
    // one_dimensional_floats();
    // console.groupEnd();

    // console.group(
    //     '%cTWO DIMENSIONAL',
    //     'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
    // );
    // two_dimensional_floats();
    // console.groupEnd();
})();
